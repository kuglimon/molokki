use flate2::read::GzDecoder;
// Documentation here is based on and copied from:
// https://falloutmods.fandom.com/wiki/SAVE.DAT_File_Format
//
//
use nom::{
    bytes::streaming::{take, take_until},
    combinator::{flat_map, map},
    error::{Error, ErrorKind},
    multi::count,
    number::streaming::{be_i32, be_u16, be_u32, be_u8},
    sequence::tuple,
    IResult,
};

use bitflags::bitflags;

use std::io::Read;
use std::str;

const SCRIPT_GROUP_COUNT: u32 = 5;
const SCRIPTS_IN_GROUP: u32 = 16;

enum MapVersion {
    Fallout1 = 19,
    Fallout2 = 20,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SaveHeader {
    pub magic: String,
    pub version: u32,
    pub release_type: u8,
    pub name: String,
    pub save_name: String,
    pub save_day: u16,
    pub save_month: u16,
    pub save_year: u16,
    pub ingame_time: u32,
    pub ingame_month: u16,
    pub ingame_year: u16,
    pub ingame_day: u16,
    pub ingame_ticks: u32,
    pub current_map: u32,
    pub map_name: String,
    pub bitmap: Vec<u8>,
    pub void: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DatFile {
    pub data_size: u32,
    pub tree_size: u32,
}

// ASCII (C-style) strings end with a 0 byte - the hex value 0x0, not the character '0'. So, if the
// length of an ASCII string is 32, it can contain 31 characters with 0x0 at the end.
//
// Parser will try to consume the requested size and the resulting string will only contain data up
// to the first null terminator.
pub fn ascii_string(size: usize) -> impl Fn(&[u8]) -> IResult<&[u8], String> {
    move |input| {
        flat_map(take_until("\0"), |string_bytes: &[u8]| {
            map(take(size - string_bytes.len()), |_| {
                str::from_utf8(string_bytes)
                    .expect("expected a valid fallout ascii string")
                    .to_string()
            })
        })(input)
    }
}

pub fn save_name(input: &[u8]) -> IResult<&[u8], String> {
    ascii_string(30)(input)
}

pub fn map_name(input: &[u8]) -> IResult<&[u8], String> {
    ascii_string(16)(input)
}

// On Steam Windows there's some extra 6 bytes of crap after the 18 byte header.
pub fn header(input: &[u8]) -> IResult<&[u8], SaveHeader> {
    map(
        tuple((
            take(18u32),
            take(6u32),
            be_u32,
            be_u8,
            ascii_string(32),
            save_name,
            be_u16,
            be_u16,
            be_u16,
            be_u32,
            be_u16,
            be_u16,
            be_u16,
            be_u32,
            be_u32,
            map_name,
            take(29792u32),
            take(128u32),
        )),
        |(
            magic,
            _,
            version,
            release_type,
            name,
            save_name,
            save_day,
            save_month,
            save_year,
            ingame_time,
            ingame_month,
            ingame_day,
            ingame_year,
            ingame_ticks,
            current_map,
            map_name,
            bitmap,
            void,
        )| {
            SaveHeader {
                magic: str::from_utf8(magic)
                    .expect("'FALLOUT SAVE FILE ' header magic ascii text expected")
                    .to_string(),
                version,
                release_type,
                name,
                save_name,
                save_day,
                save_month,
                save_year,
                ingame_time,
                ingame_month,
                ingame_year,
                ingame_day,
                ingame_ticks,
                current_map,
                map_name,
                bitmap: bitmap.to_vec(),
                void: void.to_vec(),
            }
        },
    )(input)
}

bitflags! {
    #[derive(Clone, Debug, PartialEq)]
    pub struct MapFlags: i32 {
        const IsMapSave = 0b00000001;
        const HasElevationAtLevel0 = 0b00000010;
        const HasElevationAtLevel1 = 0b00000100;
        const HasElevationAtLevel2 = 0b00001000;

        // More flags exist, but I don't know what they are for. This allows those to exist.
        const _ = !0;
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapHeader {
    pub version: u32,
    pub filename: String,
    pub default_player_position: i32,
    pub default_player_elevation: i32,
    pub default_player_orientation: i32,
    pub local_variable_count: i32,
    pub flags: MapFlags,
    pub darkness: i32,
    pub global_variable_count: i32,
    pub id: i32,
    pub ticks: u32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapVariables {
    pub global_variables: Vec<u32>,
    pub local_variables: Vec<u32>,
}

// A lot of the fields are unknown. We've left them in the struct to make it obvious what the
// format is. Rather than having the parser jump over some random bytes. This way you don't have to
// jump around from the sources to the internet to check why we're skipping some offsets.
#[derive(Clone, Debug, PartialEq)]
pub struct Script {
    pub pid: u32,

    // Not used, I guess it should be -1 always according to some documentation.
    pub next_script: i32,
    pub trigger_type: i32,
    pub radius: i32,
    pub flags: i32,
    pub id: i32,
    pub _unknown5: i32,
    pub object_id: i32,

    // Should be -1 in map files and set to some value in saves
    pub local_variable_offset: i32,

    // Should be 0 in map files and set to some value in saves
    pub local_variable_count: i32,

    pub _unknown9: i32,
    pub _unknown10: i32,
    pub _unknown11: i32,
    pub _unknown13: i32,
    pub _unknown14: i32,
    pub _unknown15: i32,
    pub _unknown16: i32,
}

// maps are laid out on 100x100 grid for both the floor and the roof. Each tile is 2 bytes. Floor
// and roof tiles alternate in the sequence.
fn tile_size_in_bytes(map_flags: &MapFlags) -> u32 {
    let mut bytes = 0;

    const ELEVATION_TILE_SIZE_BYTES: u32 = 100 * 100 * 2 * 2;

    if map_flags.contains(MapFlags::HasElevationAtLevel0) {
        bytes += ELEVATION_TILE_SIZE_BYTES;
    }

    if map_flags.contains(MapFlags::HasElevationAtLevel1) {
        bytes += ELEVATION_TILE_SIZE_BYTES;
    }

    if map_flags.contains(MapFlags::HasElevationAtLevel2) {
        bytes += ELEVATION_TILE_SIZE_BYTES;
    }

    bytes
}

pub fn dat2(input: &[u8]) -> (MapHeader, MapVariables, Vec<Script>) {
    let header = map(
        tuple((
            be_u32,
            map_name,
            be_i32,
            be_i32,
            be_i32,
            be_i32,
            be_i32,
            be_i32,
            be_i32,
            be_i32,
            be_u32,
            take(4u32 * 44u32), // unknown mystery bytes
        )),
        |(
            version,
            filename,
            default_player_position,
            default_player_elevation,
            default_player_orientation,
            local_variable_count,
            flags,
            darkness,
            global_variable_count,
            id,
            ticks,
            _,
        )| MapHeader {
            version,
            filename,
            default_player_position,
            default_player_elevation,
            default_player_orientation,
            flags: MapFlags::from_bits(flags).expect("should have parsed map flags"),
            darkness,
            global_variable_count,
            id,
            ticks,
            local_variable_count,
        },
    )(input);

    let (input, header) = header.unwrap();

    let global_variable_count: usize = header.global_variable_count.try_into().unwrap();
    let local_variable_count: usize = header.local_variable_count.try_into().unwrap();

    let map_variables = map(
        tuple::<_, _, (_, ErrorKind), _>((
            count(be_u32, global_variable_count),
            count(be_u32, local_variable_count),
        )),
        |(global_variables, local_variables)| MapVariables {
            global_variables,
            local_variables,
        },
    )(input);

    // Consume tiles
    // FIXME: Actually parse the tiles rather than discarding them
    // FIXME: I think this might be fucked/wrong
    let (input, _) =
        take::<_, _, (_, ErrorKind)>(tile_size_in_bytes(&header.flags))(input).unwrap();

    let scripts = flat_map(be_u32, |script_count| {
        // FIXME: make a parser for script counts rather than asserting here and return a parse
        // error, rather than panic
        assert!(
            script_count <= SCRIPTS_IN_GROUP,
            "script sections should not have more than 16 scripts"
        );

        println!("found {script_count} scripts");

        map(
            tuple::<_, _, (_, ErrorKind), _>((
                be_u32, be_i32, be_i32, be_i32, be_i32, be_i32, be_i32, be_i32, be_i32, be_i32,
                be_i32, be_i32, be_i32, be_i32, be_i32, be_i32, be_i32, be_i32,
            )),
            |(
                pid,
                next_script,
                trigger_type,
                radius,
                flags,
                id,
                _unknown5,
                object_id,
                local_variable_offset,
                local_variable_count,
                _unknown9,
                _unknown10,
                _unknown11,
                _unknown12,
                _unknown13,
                _unknown14,
                _unknown15,
                _unknown16,
            )| Script {
                pid,
                next_script,
                trigger_type,
                radius,
                flags,
                id,
                _unknown5,
                object_id,
                local_variable_offset,
                local_variable_count,
                _unknown9,
                _unknown10,
                _unknown11,
                _unknown13,
                _unknown14,
                _unknown15,
                _unknown16,
            },
        )
    })(input);

    (header, map_variables.unwrap().1, vec![scripts.unwrap().1])
}

pub fn try_decompress_dat2(input: Vec<u8>) -> Vec<u8> {
    // decompress if needed
    if &input[..2] == &[0x1f, 0x8b] {
        let mut decompressed: Vec<u8> = Vec::new();
        let mut decoder = GzDecoder::new(&input[..]);
        decoder
            .read_to_end(&mut decompressed)
            .expect("should have decompressed dat2");

        return decompressed;
    }

    input
}

fn main() {
    println!("Ain't doing shit yet");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Early/midgame save with NCR npcs on aggro
    const SLOT01_SAVE: &[u8] = include_bytes!("../saves/SLOT01/SAVE.DAT");
    const NCR1_SAVE: &[u8] = include_bytes!("../saves/SLOT01/NCR1.SAV");

    #[test]
    fn headers() {
        let (_bytes, save_header) = header(&SLOT01_SAVE).unwrap();

        // assert_eq!(&b""[..], bytes, "should have consumed all bytes");

        assert_eq!(save_header.magic, "FALLOUT SAVE FILE\0".to_string());
        assert_eq!(save_header.version, 65538);
        assert_eq!(save_header.release_type, 82);
        assert_eq!(save_header.name, "diglet".to_string());
        assert_eq!(save_header.save_name, "start".to_string());
        assert_eq!(save_header.save_day, 2);
        assert_eq!(save_header.save_month, 6);
        assert_eq!(save_header.save_year, 2024);
        assert_eq!(save_header.ingame_time, 68);
        assert_eq!(save_header.ingame_month, 6);
        assert_eq!(save_header.ingame_year, 2242);
        assert_eq!(save_header.ingame_day, 13);
        assert_eq!(save_header.ingame_ticks, 279545357);
        assert_eq!(save_header.current_map, 46);
        assert_eq!(save_header.map_name, "NCRENT.sav".to_string());
    }

    #[test]
    fn decompresses_dat2_files() {
        let decompressed = try_decompress_dat2(NCR1_SAVE.to_vec());

        assert_eq!(
            357576,
            decompressed.len(),
            "should have decompressed gzip dat file"
        );
    }

    #[test]
    fn parses_map_save() {
        let decompressed = try_decompress_dat2(NCR1_SAVE.to_vec());
        let (map_save, map_variables, scripts) = dat2(&decompressed);

        assert_eq!(map_save.version, MapVersion::Fallout2 as u32);
        assert_eq!(map_save.filename, "NCR1.SAV".to_string());
        assert_eq!(map_save.default_player_position, 13915);
        assert_eq!(map_save.default_player_elevation, 0);
        assert_eq!(map_save.default_player_orientation, 0);
        assert_eq!(map_save.local_variable_count, 739);
        assert!(map_save.flags.intersects(
            MapFlags::IsMapSave
                | MapFlags::HasElevationAtLevel0
                | MapFlags::HasElevationAtLevel1
                | MapFlags::HasElevationAtLevel2
        ));
        assert_eq!(map_save.darkness, 13);
        assert_eq!(map_save.global_variable_count, 1);
        assert_eq!(map_save.id, 4);
        assert_eq!(map_save.ticks, 42);

        assert_eq!(map_variables.global_variables.len(), 1);
        assert_eq!(map_variables.local_variables.len(), 739);

        dbg!(scripts);
    }
}
