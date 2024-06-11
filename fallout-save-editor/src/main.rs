use flate2::read::GzDecoder;
// Documentation here is based on and copied from:
// https://falloutmods.fandom.com/wiki/SAVE.DAT_File_Format
//
// Most of the functionality relies on the F12SE implementation for reference. It seems like F12SE
// matches fairly well what falloutmods has documented. Personally I think the offset based parsing
// is fairly confusing to follow.
//
// TODO(tatu): implement wrapper type that understands binary offsets/spans per field.
// TODO(tatu): implement wrapper type that preserves original binary and provides better view
use nom::{
    bytes::streaming::{take, take_until},
    combinator::{flat_map, map},
    error::ErrorKind,
    multi::{count, fold_many_m_n},
    number::streaming::{be_i32, be_u16, be_u32, be_u8},
    sequence::tuple,
    IResult,
};

use bitflags::bitflags;

use core::fmt;
use std::io::Read;
use std::str;

const SCRIPT_GROUP_COUNT: usize = 5;
const SCRIPTS_IN_GROUP: i32 = 16;

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

// Note that the binary format of Fallout 2 map flags uses zero flags. These are problematic for
// bitflags crate and thus we invert all but the last bit, which confusingly is not a zero flag.
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
    pub script_id: i32,
    pub flags: MapFlags,
    pub darkness: i32,
    pub global_variable_count: i32,
    pub id: i32,
    pub ticks: u32,

    // Haven't found documentation for these bytes
    pub mystery_bytes: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MapVariables {
    pub global_variables: Vec<i32>,
    pub local_variables: Vec<i32>,
}

// A lot of the fields are unknown. We've left them in the struct to make it obvious what the
// format is. Rather than having the parser jump over some random bytes. This way you don't have to
// jump around from the sources to the internet to check why we're skipping some offsets.
#[derive(Clone, Debug, PartialEq)]
pub struct Script {
    pub _prefix_junk: Vec<u8>,
    pub id: i32,

    // Should be -1 in map files and set to some value in saves
    pub local_variable_offset: i32,

    // Should be 0 in map files and set to some value in saves
    pub local_variable_count: i32,

    pub script_type: ScriptTagType,
    // pub pid: u32,
    //
    // // Not used, I guess it should be -1 always according to some documentation.
    // pub next_script: i32,
    // pub trigger_type: i32,
    // pub radius: i32,
    // pub flags: i32,
    // pub _unknown5: i32,
    // pub object_id: i32,
    //
    // pub _unknown9: i32,
    // pub _unknown10: i32,
    // pub _unknown11: i32,
    // pub _unknown13: i32,
    // pub _unknown14: i32,
    // pub _unknown15: i32,
    // pub _unknown16: i32,
}

// maps are laid out on 100x100 grid for both the floor and the roof. Each tile is 2 bytes. Floor
// and roof tiles alternate in the sequence.
fn tile_size_in_bytes(map_flags: &MapFlags) -> u32 {
    let mut bytes = 0;

    // FIXME(tatu): I probably have a bug somewhere else but for some reason it seems like these
    // tiles are actually 4 bytes per tile rather than 2. I'm not sure if sfall does some changes
    // to the save files.
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

fn map_flags(input: &[u8]) -> IResult<&[u8], MapFlags> {
    map(be_u32, |raw_flags| {
        // Having 0 flags is troublesome for bitflags. This is probably overthinking. We need to
        // flip all the other bits but LSB. This breaks binary compatibility.
        MapFlags::from_bits(
            (raw_flags ^ 0xE)
                .try_into()
                .expect("flags should not overflow"),
        )
        .expect("should have parsed map flags")
    })(input)
}

pub fn map_variable_values(
    global_variable_count: usize,
    local_variable_count: usize,
) -> impl Fn(&[u8]) -> IResult<&[u8], MapVariables> {
    move |input| {
        map(
            tuple((
                count(be_i32, global_variable_count),
                count(be_i32, local_variable_count),
            )),
            |(global_variables, local_variables)| MapVariables {
                global_variables,
                local_variables,
            },
        )(input)
    }
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
            map_flags,
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
            script_id,
            flags,
            darkness,
            global_variable_count,
            id,
            ticks,
            mystery_bytes,
        )| {
            println!("flags: {:#032b}", &flags);
            println!("gvars: {global_variable_count}");
            println!("lvars: {local_variable_count}");
            MapHeader {
                version,
                filename,
                default_player_position,
                default_player_elevation,
                default_player_orientation,
                local_variable_count,
                script_id,
                flags,
                darkness,
                global_variable_count,
                id,
                ticks,
                mystery_bytes: mystery_bytes.to_vec(),
            }
        },
    )(input);

    let (input, header) = header.unwrap();

    let global_variable_count: usize = header.global_variable_count.try_into().unwrap();
    let local_variable_count: usize = header.local_variable_count.try_into().unwrap();

    let map_variables = map_variable_values(global_variable_count, local_variable_count)(input);
    let (input, map_variables) = map_variables.expect("should have parsed map variable values");

    // Consume tiles
    // FIXME: Actually parse the tiles rather than discarding them
    let (input, _) =
        take::<_, _, (_, ErrorKind)>(tile_size_in_bytes(&header.flags))(input).unwrap();

    let scripts = fold_many_m_n(
        SCRIPT_GROUP_COUNT,
        SCRIPT_GROUP_COUNT,
        script_group,
        || Vec::new(),
        |acc, scripts| {
            let size = scripts.len();
            println!("got {size} new scripts");
            [acc, scripts].concat()
        },
    )(input);

    (header, map_variables, scripts.unwrap().1)
}

pub fn script_group(input: &[u8]) -> IResult<&[u8], Vec<Script>> {
    flat_map(be_i32, |script_count| {
        println!("trying to parse {script_count} scripts");
        // FIXME: make a parser for script counts rather than asserting here and return a parse
        // error, rather than panic
        // assert!(
        //     script_count <= SCRIPTS_IN_GROUP,
        //     "script sections should not have more than {SCRIPTS_IN_GROUP} scripts"
        // );
        println!("found {script_count} scripts");

        // FIXME(tatu): this man loves unwraps
        let script_count: usize = script_count.try_into().unwrap();

        count(script, script_count)
    })(input)
}

// Defines the type of script. 0x00 and 0x02 types are rare or unused according to F12SE sources.
// TODO: breaks binary compatibility
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ScriptTagType {
    // 0x00 - s_system
    System = 0x00,

    // 0x01 spatial s_spatial
    Spatial = 0x01,

    // 0x02 items s_time
    Items = 0x02,

    // 0x03 scenery s_item
    Scenery = 0x03,

    // 0x04 critters s_critter
    Critters = 0x04,
}

impl TryFrom<u32> for ScriptTagType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        println!("Trying to construct type from tag {:#032b}", v);
        match v {
            x if x == ScriptTagType::System as u32 => Ok(ScriptTagType::System),
            x if x == ScriptTagType::Spatial as u32 => Ok(ScriptTagType::Spatial),
            x if x == ScriptTagType::Items as u32 => Ok(ScriptTagType::Items),
            x if x == ScriptTagType::Scenery as u32 => Ok(ScriptTagType::Scenery),
            x if x == ScriptTagType::Critters as u32 => Ok(ScriptTagType::Critters),
            _ => Err(()),
        }
    }
}

impl ScriptTagType {
    pub fn byte_offset(&self) -> Result<u32, UnknownScriptSizeType> {
        use ScriptTagType::*;

        match self {
            Spatial => Ok(72),
            Items => Ok(68),
            Scenery | Critters => Ok(64),
            _ => Err(UnknownScriptSizeType {
                script_type: self.clone(),
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnknownScriptSizeType {
    script_type: ScriptTagType,
}

impl fmt::Display for UnknownScriptSizeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "size is not known for scripts of type {:?}",
            self.script_type
        )
    }
}

pub fn script_type_tag(input: &[u8]) -> IResult<&[u8], ScriptTagType> {
    map(be_u32, |script_tag_raw| {
        // FIXME(tatu): Don't unwrap, map the error to nom error
        // FIXME(tatu): Find out what type this is, seems like a PID
        //
        // This type is not really defined well anywhere. It seems like PID but PID values are
        // different.
        println!("Trying to construct type from tag {:#032b}", script_tag_raw);
        println!(
            "Trying to construct type from shifted tag {:#032b}",
            script_tag_raw >> 24
        );

        ScriptTagType::try_from(script_tag_raw >> 24).unwrap()
    })(input)
}

pub fn script(input: &[u8]) -> IResult<&[u8], Script> {
    flat_map(script_type_tag, |script_type_tag| {
        map(
            tuple((
                // FIXME(tatu): These are incorrect! I keep forgetting F12SE parses by offsets and
                // never really advances anything, this parsing starts from the script type tag
                // parsing, take that offset into account.
                // Another mystery byte skip from F12SE
                take(script_type_tag.byte_offset().unwrap() - 0x30),
                be_i32,
                take(8u32),
                be_i32,
                // Another mystery byte skip from F12SE
                be_i32,
                // Consume rest of the buffer
                // take(script_type_tag.byte_offset().unwrap() - 20u32),
            )),
            move |(_prefix_junk, id, _, local_variable_count, local_variable_offset): (
                &[u8],
                i32,
                _,
                i32,
                i32,
            )|
                  -> Script {
                let script = Script {
                    _prefix_junk: _prefix_junk.to_vec(),
                    id,
                    local_variable_count,
                    script_type: script_type_tag,
                    local_variable_offset,
                };

                println!("found script {:?}", &script);
                script
            },
        )
    })(input)
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
    fn parses_ncr_downtown_map_save() {
        let decompressed = try_decompress_dat2(NCR1_SAVE.to_vec());
        let (map_save, map_variables, scripts) = dat2(&decompressed);

        assert_eq!(map_save.version, MapVersion::Fallout2 as u32);
        assert_eq!(map_save.filename, "NCR1.SAV".to_string());
        assert_eq!(map_save.default_player_position, 13915);
        assert_eq!(map_save.default_player_elevation, 0);
        assert_eq!(map_save.default_player_orientation, 0);
        assert_eq!(map_save.local_variable_count, 739);

        // NCR should only have zero elevation and this is a map save
        assert!(
            map_save
                .flags
                .contains(MapFlags::HasElevationAtLevel1 | MapFlags::HasElevationAtLevel2)
                == false
        );
        assert!(map_save
            .flags
            .contains(MapFlags::IsMapSave | MapFlags::HasElevationAtLevel0));
        assert_eq!(map_save.darkness, 1);
        assert_eq!(map_save.global_variable_count, 4);
        assert_eq!(map_save.id, 42);
        assert_eq!(map_save.ticks, 279545083);
        assert_eq!(map_save.mystery_bytes.len(), 4 * 44);

        assert_eq!(
            map_variables.global_variables.len(),
            map_save.global_variable_count.try_into().unwrap()
        );
        assert_eq!(
            map_variables.local_variables.len(),
            map_save.local_variable_count.try_into().unwrap()
        );

        dbg!(scripts);
    }
}
