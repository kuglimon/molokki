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
const SCRIPTS_IN_GROUP: usize = 16;

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
    let start = input.len();
    println!("starting from {start}");
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

    println!("at variable offset {}", start - input.len());

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
            let had = acc.len();
            println!("got {size} new scripts had {had}");
            [acc, scripts].concat()
        },
    )(input);

    (header, map_variables, scripts.unwrap().1)
}

pub fn script_group(input: &[u8]) -> IResult<&[u8], Vec<Script>> {
    let (mut input, script_count) = be_i32(input)?;

    println!("trying to parse {script_count} scripts");
    // FIXME: make a parser for script counts rather than asserting here and return a parse
    // error, rather than panic
    // assert!(
    //     script_count <= SCRIPTS_IN_GROUP,
    //     "script sections should not have more than {SCRIPTS_IN_GROUP} scripts"
    // );
    println!("found {script_count} scripts");

    // FIXME(tatu): this man loves unwraps
    let mut script_count: usize = script_count.try_into().unwrap();
    let mut scripts = Vec::new();

    while script_count > SCRIPTS_IN_GROUP {
        let (remaining_input, mut new_scripts) = map(
            tuple((
                count(script, SCRIPTS_IN_GROUP),
                take(8u32), // script check counter and possible crc check
            )),
            |(scripts, _)| scripts,
        )(input)?;

        scripts.append(&mut new_scripts);
        script_count -= SCRIPTS_IN_GROUP;

        input = remaining_input;
    }

    println!("{script_count} scripts left after parsing");

    let (input, mut new_scripts) = map(count(script, script_count), |scripts| scripts)(input)?;
    scripts.append(&mut new_scripts);

    let input = if script_count > 0 {
        let remaining_block = SCRIPTS_IN_GROUP - script_count;

        println!("{remaining_block} junk left");

        let (input, _) = tuple((
            count(read_script_block_junk, remaining_block),
            take(8u32), // script check counter and possible crc check
        ))(input)?;
        input
    } else {
        input
    };

    let curr = input.len();

    println!("at offset {curr}");

    Ok((input, scripts))
}

pub fn read_script_block_junk(input: &[u8]) -> IResult<&[u8], &[u8]> {
    flat_map(script_type_tag, |script_type_tag| {
        let junk_size = script_type_tag.junk_size();
        println!("reading junk rec {:?}", script_type_tag);
        println!("reading junk {junk_size}");
        // FIXME(tatu): record sizes include the size and we've consumed it already, so substract 4
        // bytes. This is confusing as fuck. Make something better once everything works.
        take(script_type_tag.junk_size() - 4)
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

    // FIXME(tatu): just a catch all to fallback to, value might colide with actual types
    Unknown = 0xff,
}

impl TryFrom<u32> for ScriptTagType {
    type Error = ();

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == ScriptTagType::System as u32 => Ok(ScriptTagType::System),
            x if x == ScriptTagType::Spatial as u32 => Ok(ScriptTagType::Spatial),
            x if x == ScriptTagType::Items as u32 => Ok(ScriptTagType::Items),
            x if x == ScriptTagType::Scenery as u32 => Ok(ScriptTagType::Scenery),
            x if x == ScriptTagType::Critters as u32 => Ok(ScriptTagType::Critters),
            _ => Ok(ScriptTagType::Unknown),
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

    // How is it that we know the size of junk but not the actual records but the sizes are the
    // same? This is something F12SE does, but it isn't documented, this is just called junk.
    pub fn junk_size(&self) -> usize {
        use ScriptTagType::*;

        match self {
            Spatial => 72,
            Items => 68,
            _ => 64,
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
        println!("got {}", (script_tag_raw as i32) >> 24);
        ScriptTagType::try_from(script_tag_raw >> 24).unwrap()
    })(input)
}

pub fn script(input: &[u8]) -> IResult<&[u8], Script> {
    // FIXME(tatu): We should peek script tag type and then parse the whole record as its own
    // buffer. All the offset calculations are now super confusing as we've consumed part of the
    // record and then carry that in all calculations.
    flat_map(script_type_tag, |script_type_tag| {
        let offset = input.len();
        println!("at scripts offset {:?}", offset);
        let record_size = script_type_tag.byte_offset().unwrap();
        // TODO(tatu): Kinda bad as we need to keep this 20 bytes in sync with what we've read. I
        // think a better option is to slice the input at record size, parse that while discarding
        // the rest and then manually advance the input buffer.
        let junk_size = record_size - (record_size - 0x38 + 20u32 + 4u32);
        println!("script suffix junk size {:?}", junk_size);
        map(
            tuple((
                // Another mystery byte skip from F12SE
                take(record_size - 0x38),
                be_i32,
                take(8u32),
                be_i32,
                // Another mystery byte skip from F12SE
                be_i32,
                // Consume rest of the buffer
                take(junk_size),
            )),
            move |(_prefix_junk, id, _, local_variable_offset, local_variable_count, _): (
                &[u8],
                i32,
                _,
                i32,
                i32,
                _,
            )|
                  -> Script {
                let script = Script {
                    _prefix_junk: _prefix_junk.to_vec(),
                    id,
                    local_variable_count,
                    script_type: script_type_tag,
                    local_variable_offset,
                };

                // println!("found script {:?}", &script);
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
    const AUTOMAP_SAVE: &[u8] = include_bytes!("../saves/SLOT01/AUTOMAP.SAV");

    const ARBRIDGE_SAVE: &[u8] = include_bytes!("../saves/SLOT01/ARBRIDGE.SAV");
    const ARCAVES_SAVE: &[u8] = include_bytes!("../saves/SLOT01/ARCAVES.SAV");
    const ARGARDEN_SAVE: &[u8] = include_bytes!("../saves/SLOT01/ARGARDEN.SAV");
    const ARTEMPLE_SAVE: &[u8] = include_bytes!("../saves/SLOT01/ARTEMPLE.SAV");
    const ARVILLAG_SAVE: &[u8] = include_bytes!("../saves/SLOT01/ARVILLAG.SAV");
    const BROKEN1_SAVE: &[u8] = include_bytes!("../saves/SLOT01/BROKEN1.SAV");
    const BROKEN2_SAVE: &[u8] = include_bytes!("../saves/SLOT01/BROKEN2.SAV");
    const DENBUS1_SAVE: &[u8] = include_bytes!("../saves/SLOT01/DENBUS1.SAV");
    const DENBUS2_SAVE: &[u8] = include_bytes!("../saves/SLOT01/DENBUS2.SAV");
    const GECKJUNK_SAVE: &[u8] = include_bytes!("../saves/SLOT01/GECKJUNK.SAV");
    const GECKPWPL_SAVE: &[u8] = include_bytes!("../saves/SLOT01/GECKPWPL.SAV");
    const GECKSETL_SAVE: &[u8] = include_bytes!("../saves/SLOT01/GECKSETL.SAV");
    const GECKTUNL_SAVE: &[u8] = include_bytes!("../saves/SLOT01/GECKTUNL.SAV");
    const GSTCAV1_SAVE: &[u8] = include_bytes!("../saves/SLOT01/GSTCAV1.SAV");
    const GSTCAV2_SAVE: &[u8] = include_bytes!("../saves/SLOT01/GSTCAV2.SAV");
    const GSTFARM_SAVE: &[u8] = include_bytes!("../saves/SLOT01/GSTFARM.SAV");
    const KLACANYN_SAVE: &[u8] = include_bytes!("../saves/SLOT01/KLACANYN.SAV");
    const KLADWTWN_SAVE: &[u8] = include_bytes!("../saves/SLOT01/KLADWTWN.SAV");
    const KLAGRAZ_SAVE: &[u8] = include_bytes!("../saves/SLOT01/KLAGRAZ.SAV");
    const KLATOXCV_SAVE: &[u8] = include_bytes!("../saves/SLOT01/KLATOXCV.SAV");
    const KLATRAP_SAVE: &[u8] = include_bytes!("../saves/SLOT01/KLATRAP.SAV");
    const MODGARD_SAVE: &[u8] = include_bytes!("../saves/SLOT01/MODGARD.SAV");
    const MODINN_SAVE: &[u8] = include_bytes!("../saves/SLOT01/MODINN.SAV");
    const MODMAIN_SAVE: &[u8] = include_bytes!("../saves/SLOT01/MODMAIN.SAV");
    const NCR1_SAVE: &[u8] = include_bytes!("../saves/SLOT01/NCR1.SAV");
    const NCRENT_SAVE: &[u8] = include_bytes!("../saves/SLOT01/NCRENT.SAV");
    const NEWR1_SAVE: &[u8] = include_bytes!("../saves/SLOT01/NEWR1.SAV");
    const NEWR2_SAVE: &[u8] = include_bytes!("../saves/SLOT01/NEWR2.SAV");
    const NEWR3_SAVE: &[u8] = include_bytes!("../saves/SLOT01/NEWR3.SAV");
    const NEWRST_SAVE: &[u8] = include_bytes!("../saves/SLOT01/NEWRST.SAV");
    const RAIDERS1_SAVE: &[u8] = include_bytes!("../saves/SLOT01/RAIDERS1.SAV");
    const RAIDERS2_SAVE: &[u8] = include_bytes!("../saves/SLOT01/RAIDERS2.SAV");
    const REDDOWN_SAVE: &[u8] = include_bytes!("../saves/SLOT01/REDDOWN.SAV");
    const REDMENT_SAVE: &[u8] = include_bytes!("../saves/SLOT01/REDMENT.SAV");
    const REDMTUN_SAVE: &[u8] = include_bytes!("../saves/SLOT01/REDMTUN.SAV");
    const REDWAME_SAVE: &[u8] = include_bytes!("../saves/SLOT01/REDWAME.SAV");
    const V15ENT_SAVE: &[u8] = include_bytes!("../saves/SLOT01/V15ENT.SAV");
    const V15SENT_SAVE: &[u8] = include_bytes!("../saves/SLOT01/V15SENT.SAV");
    const VCTYCOCL_SAVE: &[u8] = include_bytes!("../saves/SLOT01/VCTYCOCL.SAV");
    const VCTYCTYD_SAVE: &[u8] = include_bytes!("../saves/SLOT01/VCTYCTYD.SAV");
    const VCTYDWTN_SAVE: &[u8] = include_bytes!("../saves/SLOT01/VCTYDWTN.SAV");
    const VCTYVLT_SAVE: &[u8] = include_bytes!("../saves/SLOT01/VCTYVLT.SAV");

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

        assert_eq!(scripts.len(), 85);
    }

    #[test]
    fn parses_arroyo_bridge_map_save() {
        let decompressed = try_decompress_dat2(ARBRIDGE_SAVE.to_vec());
        let (_, _, scripts) = dat2(&decompressed);

        assert_eq!(scripts.len(), 3);
    }

    #[test]
    fn parses_raiders_map_1_map_save() {
        let decompressed = try_decompress_dat2(RAIDERS1_SAVE.to_vec());
        let (_, _, scripts) = dat2(&decompressed);

        assert_eq!(scripts.len(), 0);
    }

    #[test]
    fn parses_arroy_caves_map_save() {
        let decompressed = try_decompress_dat2(ARCAVES_SAVE.to_vec());
        let (_, _, scripts) = dat2(&decompressed);

        assert_eq!(scripts.len(), 26);
    }

    #[test]
    fn parses_arroy_village_garden_map_save() {
        let decompressed = try_decompress_dat2(ARGARDEN_SAVE.to_vec());
        let (_, _, scripts) = dat2(&decompressed);

        assert_eq!(scripts.len(), 10);
    }

    // According to https://fallout.fandom.com/wiki/ARTEMPLE.SSL this is arroy caves but so is
    // ARCAVES.SAVE... Maybe this is the temple?
    #[test]
    fn parses_arroy_temple_map_save() {
        let decompressed = try_decompress_dat2(ARTEMPLE_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 15);
        assert_eq!(header.global_variable_count, 0);

        assert_eq!(variables.local_variables.len(), 15);
        assert_eq!(variables.global_variables.len(), 0);

        assert_eq!(scripts.len(), 3);
    }

    #[test]
    fn parses_arroy_village_map_save() {
        let decompressed = try_decompress_dat2(ARVILLAG_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 296);
        assert_eq!(header.global_variable_count, 5);

        assert_eq!(variables.local_variables.len(), 296);
        assert_eq!(variables.global_variables.len(), 5);

        assert_eq!(scripts.len(), 141);
    }

    #[test]
    fn parses_broken_hills_village_1_map_save() {
        let decompressed = try_decompress_dat2(BROKEN1_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 913);
        assert_eq!(header.global_variable_count, 31);

        assert_eq!(variables.local_variables.len(), 913);
        assert_eq!(variables.global_variables.len(), 31);

        assert_eq!(scripts.len(), 98);
    }

    #[test]
    fn parses_broken_hills_village_2_map_save() {
        let decompressed = try_decompress_dat2(BROKEN2_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 521);
        assert_eq!(header.global_variable_count, 25);

        assert_eq!(variables.local_variables.len(), 521);
        assert_eq!(variables.global_variables.len(), 25);

        assert_eq!(scripts.len(), 120);
    }

    #[test]
    fn parses_the_den_business_area_1_map_save() {
        let decompressed = try_decompress_dat2(DENBUS1_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 798);
        assert_eq!(header.global_variable_count, 10);

        assert_eq!(variables.local_variables.len(), 798);
        assert_eq!(variables.global_variables.len(), 10);

        assert_eq!(scripts.len(), 95);
    }

    #[test]
    fn parses_the_den_business_area_2_map_save() {
        let decompressed = try_decompress_dat2(DENBUS2_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 853);
        assert_eq!(header.global_variable_count, 13);

        assert_eq!(variables.local_variables.len(), 853);
        assert_eq!(variables.global_variables.len(), 13);

        assert_eq!(scripts.len(), 145);
    }

    #[test]
    fn parses_gecko_junkyard_map_save() {
        let decompressed = try_decompress_dat2(GECKJUNK_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 207);
        assert_eq!(header.global_variable_count, 8);

        assert_eq!(variables.local_variables.len(), 207);
        assert_eq!(variables.global_variables.len(), 8);

        assert_eq!(scripts.len(), 21);
    }

    #[test]
    fn parses_gecko_power_plant_map_save() {
        let decompressed = try_decompress_dat2(GECKPWPL_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 327);
        assert_eq!(header.global_variable_count, 20);

        assert_eq!(variables.local_variables.len(), 327);
        assert_eq!(variables.global_variables.len(), 20);

        assert_eq!(scripts.len(), 50);
    }

    #[test]
    fn parses_gecko_settlement_map_save() {
        let decompressed = try_decompress_dat2(GECKSETL_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 332);
        assert_eq!(header.global_variable_count, 9);

        assert_eq!(variables.local_variables.len(), 332);
        assert_eq!(variables.global_variables.len(), 9);

        assert_eq!(scripts.len(), 34);
    }

    #[test]
    fn parses_gecko_tunnel_map_map_save() {
        let decompressed = try_decompress_dat2(GECKTUNL_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 109);
        assert_eq!(header.global_variable_count, 8);

        assert_eq!(variables.local_variables.len(), 109);
        assert_eq!(variables.global_variables.len(), 8);

        assert_eq!(scripts.len(), 12);
    }

    #[test]
    fn parses_gstcav1_map_save() {
        let decompressed = try_decompress_dat2(GSTCAV1_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 19);
        assert_eq!(header.global_variable_count, 0);

        assert_eq!(variables.local_variables.len(), 19);
        assert_eq!(variables.global_variables.len(), 0);

        assert_eq!(scripts.len(), 6);
    }

    #[test]
    fn parses_gstcav2_map_save() {
        let decompressed = try_decompress_dat2(GSTCAV2_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 61);
        assert_eq!(header.global_variable_count, 0);

        assert_eq!(variables.local_variables.len(), 61);
        assert_eq!(variables.global_variables.len(), 0);

        assert_eq!(scripts.len(), 7);
    }

    #[test]
    fn parses_gstfarm_map_save() {
        let decompressed = try_decompress_dat2(GSTFARM_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 25);
        assert_eq!(header.global_variable_count, 1);

        assert_eq!(variables.local_variables.len(), 25);
        assert_eq!(variables.global_variables.len(), 1);

        assert_eq!(scripts.len(), 42);
    }

    #[test]
    fn parses_klacanyn_map_save() {
        let decompressed = try_decompress_dat2(KLACANYN_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 20);
        assert_eq!(header.global_variable_count, 19);

        assert_eq!(variables.local_variables.len(), 20);
        assert_eq!(variables.global_variables.len(), 19);

        assert_eq!(scripts.len(), 1);
    }

    #[test]
    fn parses_klamath_village_map_save() {
        let decompressed = try_decompress_dat2(KLADWTWN_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 690);
        assert_eq!(header.global_variable_count, 19);

        assert_eq!(variables.local_variables.len(), 690);
        assert_eq!(variables.global_variables.len(), 19);

        assert_eq!(scripts.len(), 85);
    }

    #[test]
    fn parses_klamath_graze_map_map_save() {
        let decompressed = try_decompress_dat2(KLAGRAZ_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 20);
        assert_eq!(header.global_variable_count, 20);

        assert_eq!(variables.local_variables.len(), 20);
        assert_eq!(variables.global_variables.len(), 20);

        assert_eq!(scripts.len(), 5);
    }

    #[test]
    fn parses_arroyo_bridge_1_map_save() {
        let decompressed = try_decompress_dat2(KLATOXCV_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 38);
        assert_eq!(header.global_variable_count, 18);

        assert_eq!(variables.local_variables.len(), 38);
        assert_eq!(variables.global_variables.len(), 18);

        assert_eq!(scripts.len(), 28);
    }

    #[test]
    fn parses_klatrap_map_save() {
        let decompressed = try_decompress_dat2(KLATRAP_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 10);
        assert_eq!(header.global_variable_count, 0);

        assert_eq!(variables.local_variables.len(), 10);
        assert_eq!(variables.global_variables.len(), 0);

        assert_eq!(scripts.len(), 3);
    }

    #[test]
    fn parses_modgard_map_save() {
        let decompressed = try_decompress_dat2(MODGARD_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 0);
        assert_eq!(header.global_variable_count, 1);

        assert_eq!(variables.local_variables.len(), 0);
        assert_eq!(variables.global_variables.len(), 1);

        assert_eq!(scripts.len(), 0);
    }

    #[test]
    fn parses_modinn_map_save() {
        let decompressed = try_decompress_dat2(MODINN_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 264);
        assert_eq!(header.global_variable_count, 2);

        assert_eq!(variables.local_variables.len(), 264);
        assert_eq!(variables.global_variables.len(), 2);

        assert_eq!(scripts.len(), 41);
    }

    #[test]
    fn parses_modmain_map_save() {
        let decompressed = try_decompress_dat2(MODMAIN_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 417);
        assert_eq!(header.global_variable_count, 4);

        assert_eq!(variables.local_variables.len(), 417);
        assert_eq!(variables.global_variables.len(), 4);

        assert_eq!(scripts.len(), 55);
    }

    #[test]
    fn parses_ncr_map_entrance_map_save() {
        let decompressed = try_decompress_dat2(NCRENT_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 634);
        assert_eq!(header.global_variable_count, 7);

        assert_eq!(variables.local_variables.len(), 634);
        assert_eq!(variables.global_variables.len(), 7);

        assert_eq!(scripts.len(), 84);
    }

    #[test]
    fn parses_newr1_map_save() {
        let decompressed = try_decompress_dat2(NEWR1_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 858);
        assert_eq!(header.global_variable_count, 1);

        assert_eq!(variables.local_variables.len(), 858);
        assert_eq!(variables.global_variables.len(), 1);

        assert_eq!(scripts.len(), 191);
    }

    #[test]
    fn parses_newr2_map_save() {
        let decompressed = try_decompress_dat2(NEWR2_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 949);
        assert_eq!(header.global_variable_count, 1);

        assert_eq!(variables.local_variables.len(), 949);
        assert_eq!(variables.global_variables.len(), 1);

        assert_eq!(scripts.len(), 215);
    }

    #[test]
    fn parses_newr3_map_save() {
        let decompressed = try_decompress_dat2(NEWR3_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 221);
        assert_eq!(header.global_variable_count, 0);

        assert_eq!(variables.local_variables.len(), 221);
        assert_eq!(variables.global_variables.len(), 0);

        assert_eq!(scripts.len(), 56);
    }

    #[test]
    fn parses_newrst_map_save() {
        let decompressed = try_decompress_dat2(NEWRST_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 319);
        assert_eq!(header.global_variable_count, 2);

        assert_eq!(variables.local_variables.len(), 319);
        assert_eq!(variables.global_variables.len(), 2);

        assert_eq!(scripts.len(), 73);
    }

    #[test]
    fn parses_raiders_map_2_map_save() {
        let decompressed = try_decompress_dat2(RAIDERS2_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 337);
        assert_eq!(header.global_variable_count, 3);

        assert_eq!(variables.local_variables.len(), 337);
        assert_eq!(variables.global_variables.len(), 3);

        assert_eq!(scripts.len(), 177);
    }

    #[test]
    fn parses_denbus2_map_save() {
        let decompressed = try_decompress_dat2(REDDOWN_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 757);
        assert_eq!(header.global_variable_count, 5);

        assert_eq!(variables.local_variables.len(), 757);
        assert_eq!(variables.global_variables.len(), 5);

        assert_eq!(scripts.len(), 96);
    }

    #[test]
    fn parses_redding_mine_entrance_map_save() {
        let decompressed = try_decompress_dat2(REDMENT_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 808);
        assert_eq!(header.global_variable_count, 11);

        assert_eq!(variables.local_variables.len(), 808);
        assert_eq!(variables.global_variables.len(), 11);

        assert_eq!(scripts.len(), 94);
    }

    #[test]
    fn parses_arroyo_caves_map_save() {
        let decompressed = try_decompress_dat2(REDMTUN_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 6);
        assert_eq!(header.global_variable_count, 0);

        assert_eq!(variables.local_variables.len(), 6);
        assert_eq!(variables.global_variables.len(), 0);

        assert_eq!(scripts.len(), 13);
    }

    #[test]
    fn parses_arroyo_caves_2_map_save() {
        let decompressed = try_decompress_dat2(REDWAME_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 185);
        assert_eq!(header.global_variable_count, 16);

        assert_eq!(variables.local_variables.len(), 185);
        assert_eq!(variables.global_variables.len(), 16);

        assert_eq!(scripts.len(), 35);
    }

    #[test]
    fn parses_v15ent_map_save() {
        let decompressed = try_decompress_dat2(V15ENT_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 106);
        assert_eq!(header.global_variable_count, 2);

        assert_eq!(variables.local_variables.len(), 106);
        assert_eq!(variables.global_variables.len(), 2);

        assert_eq!(scripts.len(), 18);
    }

    #[test]
    fn parses_vault15_secret_entrance_map_map_save() {
        let decompressed = try_decompress_dat2(V15SENT_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 60);
        assert_eq!(header.global_variable_count, 1);

        assert_eq!(variables.local_variables.len(), 60);
        assert_eq!(variables.global_variables.len(), 1);

        assert_eq!(scripts.len(), 7);
    }

    #[test]
    fn parses_arroyo_bridge_2_map_save() {
        let decompressed = try_decompress_dat2(VCTYCOCL_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 400);
        assert_eq!(header.global_variable_count, 1);

        assert_eq!(variables.local_variables.len(), 400);
        assert_eq!(variables.global_variables.len(), 1);

        assert_eq!(scripts.len(), 52);
    }

    #[test]
    fn parses_vctyctyd_map_save() {
        let decompressed = try_decompress_dat2(VCTYCTYD_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 277);
        assert_eq!(header.global_variable_count, 7);

        assert_eq!(variables.local_variables.len(), 277);
        assert_eq!(variables.global_variables.len(), 7);

        assert_eq!(scripts.len(), 49);
    }

    #[test]
    fn parses_arroyo_bridge_3_map_save() {
        let decompressed = try_decompress_dat2(VCTYDWTN_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 460);
        assert_eq!(header.global_variable_count, 9);

        assert_eq!(variables.local_variables.len(), 460);
        assert_eq!(variables.global_variables.len(), 9);

        assert_eq!(scripts.len(), 74);
    }

    #[test]
    fn parses_vault_city_vault_map_save() {
        let decompressed = try_decompress_dat2(VCTYVLT_SAVE.to_vec());
        let (header, variables, scripts) = dat2(&decompressed);

        assert_eq!(header.local_variable_count, 87);
        assert_eq!(header.global_variable_count, 5);

        assert_eq!(variables.local_variables.len(), 87);
        assert_eq!(variables.global_variables.len(), 5);

        assert_eq!(scripts.len(), 25);
    }
}
