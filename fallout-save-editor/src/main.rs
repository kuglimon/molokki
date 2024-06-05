// Documentation here is based on and copied from:
// https://falloutmods.fandom.com/wiki/SAVE.DAT_File_Format
//
//
use nom::{
    bytes::streaming::take,
    combinator::map,
    number::streaming::{be_u16, be_u32, be_u8},
    sequence::tuple,
    IResult,
};

use std::{fs, str};

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

// ASCII (C-style) strings end with a 0 byte - the hex value 0x0, not the character '0'. So, if the
// length of an ASCII string is 32, it can contain 31 characters with 0x0 at the end.
pub fn ascii_string(input: &[u8]) -> IResult<&[u8], String> {
    map(tuple((take(31u32), take(1u32))), |(name, _)| {
        str::from_utf8(name)
            .expect("expected a valid fallout ascii string")
            .to_string()
    })(input)
}

pub fn save_name(input: &[u8]) -> IResult<&[u8], String> {
    map(tuple((take(29u32), take(1u32))), |(name, _)| {
        str::from_utf8(name)
            .expect("expected a valid fallout ascii string")
            .to_string()
    })(input)
}

pub fn map_name(input: &[u8]) -> IResult<&[u8], String> {
    map(tuple((take(15u32), take(1u32))), |(name, _)| {
        str::from_utf8(name)
            .expect("expected a valid fallout ascii string")
            .to_string()
    })(input)
}

// On Steam Windows there's some extra 6 bytes of crap after the 18 byte header.
pub fn header(input: &[u8]) -> IResult<&[u8], SaveHeader> {
    map(
        tuple((
            take(18u32),
            take(6u32),
            be_u32,
            be_u8,
            ascii_string,
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

fn main() {
    println!("Ain't doing shit yet");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Early/midgame save with NCR npcs on aggro
    const SLOT01_SAVE: &[u8] = include_bytes!("../saves/SLOT01/SAVE.DAT");

    #[test]
    fn headers() {
        let (_bytes, save_header) = header(&SLOT01_SAVE).unwrap();

        // assert_eq!(&b""[..], bytes, "should have consumed all bytes");

        assert_eq!(save_header.magic, "FALLOUT SAVE FILE\0".to_string());
        assert_eq!(save_header.version, 65538);
        assert_eq!(save_header.release_type, 82);
        assert_eq!(
            save_header.name,
            "diglet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".to_string()
        );
        assert_eq!(
            save_header.save_name,
            "start\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0".to_string()
        );
        assert_eq!(save_header.save_day, 2);
        assert_eq!(save_header.save_month, 6);
        assert_eq!(save_header.save_year, 2024);
        assert_eq!(save_header.ingame_time, 68);
        assert_eq!(save_header.ingame_month, 6);
        assert_eq!(save_header.ingame_year, 2242);
        assert_eq!(save_header.ingame_day, 13);
        assert_eq!(save_header.ingame_ticks, 279545357);
        assert_eq!(save_header.current_map, 46);
        assert_eq!(save_header.map_name, "NCRENT.sav\0\0\0\0\0".to_string());
    }
}
