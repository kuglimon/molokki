use fallout_save_editor::parser::{
    header, map_save, try_gunzip_buffer, MapFlags, MapVersion, ScriptTagType,
};

// Early/midgame save with NCR npcs on aggro
const SLOT01_SAVE: &[u8] = include_bytes!("../saves/SLOT01/SAVE.DAT");

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
    let decompressed = try_gunzip_buffer(NCR1_SAVE.to_vec());

    assert_eq!(
        357576,
        decompressed.len(),
        "should have decompressed gzip dat file"
    );
}

#[test]
fn parses_ncr_downtown_map_save() {
    let decompressed = try_gunzip_buffer(NCR1_SAVE.to_vec());
    let (map_save, map_variables, scripts) = map_save(&decompressed);

    assert_eq!(map_save.version, MapVersion::Fallout2);
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
    let decompressed = try_gunzip_buffer(ARBRIDGE_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    assert_eq!(scripts.len(), 3);
}

#[test]
fn parses_raiders_map_1_map_save() {
    let decompressed = try_gunzip_buffer(RAIDERS1_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    assert_eq!(scripts.len(), 0);
}

#[test]
fn parses_arroy_caves_map_save() {
    let decompressed = try_gunzip_buffer(ARCAVES_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    assert_eq!(scripts.len(), 26);
}

#[test]
fn parses_arroy_village_garden_map_save() {
    let decompressed = try_gunzip_buffer(ARGARDEN_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    assert_eq!(scripts.len(), 10);
}

// According to https://fallout.fandom.com/wiki/ARTEMPLE.SSL this is arroy caves but so is
// ARCAVES.SAVE... Maybe this is the temple?
#[test]
fn parses_arroy_temple_map_save() {
    let decompressed = try_gunzip_buffer(ARTEMPLE_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 15);
    assert_eq!(header.global_variable_count, 0);

    assert_eq!(variables.local_variables.len(), 15);
    assert_eq!(variables.global_variables.len(), 0);

    assert_eq!(scripts.len(), 3);
}

#[test]
fn parses_arroy_village_map_save() {
    let decompressed = try_gunzip_buffer(ARVILLAG_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 296);
    assert_eq!(header.global_variable_count, 5);

    assert_eq!(variables.local_variables.len(), 296);
    assert_eq!(variables.global_variables.len(), 5);

    assert_eq!(scripts.len(), 141);
}

#[test]
fn parses_broken_hills_village_1_map_save() {
    let decompressed = try_gunzip_buffer(BROKEN1_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 913);
    assert_eq!(header.global_variable_count, 31);

    assert_eq!(variables.local_variables.len(), 913);
    assert_eq!(variables.global_variables.len(), 31);

    assert_eq!(scripts.len(), 98);
}

#[test]
fn parses_broken_hills_village_2_map_save() {
    let decompressed = try_gunzip_buffer(BROKEN2_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 521);
    assert_eq!(header.global_variable_count, 25);

    assert_eq!(variables.local_variables.len(), 521);
    assert_eq!(variables.global_variables.len(), 25);

    assert_eq!(scripts.len(), 120);
}

#[test]
fn parses_the_den_business_area_1_map_save() {
    let decompressed = try_gunzip_buffer(DENBUS1_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 798);
    assert_eq!(header.global_variable_count, 10);

    assert_eq!(variables.local_variables.len(), 798);
    assert_eq!(variables.global_variables.len(), 10);

    assert_eq!(scripts.len(), 95);
}

#[test]
fn parses_the_den_business_area_2_map_save() {
    let decompressed = try_gunzip_buffer(DENBUS2_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 853);
    assert_eq!(header.global_variable_count, 13);

    assert_eq!(variables.local_variables.len(), 853);
    assert_eq!(variables.global_variables.len(), 13);

    assert_eq!(scripts.len(), 145);
}

#[test]
fn parses_gecko_junkyard_map_save() {
    let decompressed = try_gunzip_buffer(GECKJUNK_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 207);
    assert_eq!(header.global_variable_count, 8);

    assert_eq!(variables.local_variables.len(), 207);
    assert_eq!(variables.global_variables.len(), 8);

    assert_eq!(scripts.len(), 21);
}

#[test]
fn parses_gecko_power_plant_map_save() {
    let decompressed = try_gunzip_buffer(GECKPWPL_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 327);
    assert_eq!(header.global_variable_count, 20);

    assert_eq!(variables.local_variables.len(), 327);
    assert_eq!(variables.global_variables.len(), 20);

    assert_eq!(scripts.len(), 50);
}

#[test]
fn parses_gecko_settlement_map_save() {
    let decompressed = try_gunzip_buffer(GECKSETL_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 332);
    assert_eq!(header.global_variable_count, 9);

    assert_eq!(variables.local_variables.len(), 332);
    assert_eq!(variables.global_variables.len(), 9);

    assert_eq!(scripts.len(), 34);
}

#[test]
fn parses_gecko_tunnel_map_map_save() {
    let decompressed = try_gunzip_buffer(GECKTUNL_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 109);
    assert_eq!(header.global_variable_count, 8);

    assert_eq!(variables.local_variables.len(), 109);
    assert_eq!(variables.global_variables.len(), 8);

    assert_eq!(scripts.len(), 12);
}

#[test]
fn parses_gstcav1_map_save() {
    let decompressed = try_gunzip_buffer(GSTCAV1_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 19);
    assert_eq!(header.global_variable_count, 0);

    assert_eq!(variables.local_variables.len(), 19);
    assert_eq!(variables.global_variables.len(), 0);

    assert_eq!(scripts.len(), 6);
}

#[test]
fn parses_gstcav2_map_save() {
    let decompressed = try_gunzip_buffer(GSTCAV2_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 61);
    assert_eq!(header.global_variable_count, 0);

    assert_eq!(variables.local_variables.len(), 61);
    assert_eq!(variables.global_variables.len(), 0);

    assert_eq!(scripts.len(), 7);
}

#[test]
fn parses_gstfarm_map_save() {
    let decompressed = try_gunzip_buffer(GSTFARM_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 25);
    assert_eq!(header.global_variable_count, 1);

    assert_eq!(variables.local_variables.len(), 25);
    assert_eq!(variables.global_variables.len(), 1);

    assert_eq!(scripts.len(), 42);
}

#[test]
fn parses_klacanyn_map_save() {
    let decompressed = try_gunzip_buffer(KLACANYN_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 20);
    assert_eq!(header.global_variable_count, 19);

    assert_eq!(variables.local_variables.len(), 20);
    assert_eq!(variables.global_variables.len(), 19);

    assert_eq!(scripts.len(), 1);
}

#[test]
fn parses_klamath_village_map_save() {
    let decompressed = try_gunzip_buffer(KLADWTWN_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 690);
    assert_eq!(header.global_variable_count, 19);

    assert_eq!(variables.local_variables.len(), 690);
    assert_eq!(variables.global_variables.len(), 19);

    assert_eq!(scripts.len(), 85);
}

#[test]
fn parses_klamath_graze_map_map_save() {
    let decompressed = try_gunzip_buffer(KLAGRAZ_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 20);
    assert_eq!(header.global_variable_count, 20);

    assert_eq!(variables.local_variables.len(), 20);
    assert_eq!(variables.global_variables.len(), 20);

    assert_eq!(scripts.len(), 5);
}

#[test]
fn parses_arroyo_bridge_1_map_save() {
    let decompressed = try_gunzip_buffer(KLATOXCV_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 38);
    assert_eq!(header.global_variable_count, 18);

    assert_eq!(variables.local_variables.len(), 38);
    assert_eq!(variables.global_variables.len(), 18);

    assert_eq!(scripts.len(), 28);
}

#[test]
fn parses_klatrap_map_save() {
    let decompressed = try_gunzip_buffer(KLATRAP_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 10);
    assert_eq!(header.global_variable_count, 0);

    assert_eq!(variables.local_variables.len(), 10);
    assert_eq!(variables.global_variables.len(), 0);

    assert_eq!(scripts.len(), 3);
}

#[test]
fn parses_modgard_map_save() {
    let decompressed = try_gunzip_buffer(MODGARD_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 0);
    assert_eq!(header.global_variable_count, 1);

    assert_eq!(variables.local_variables.len(), 0);
    assert_eq!(variables.global_variables.len(), 1);

    assert_eq!(scripts.len(), 0);
}

#[test]
fn parses_modinn_map_save() {
    let decompressed = try_gunzip_buffer(MODINN_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 264);
    assert_eq!(header.global_variable_count, 2);

    assert_eq!(variables.local_variables.len(), 264);
    assert_eq!(variables.global_variables.len(), 2);

    assert_eq!(scripts.len(), 41);
}

#[test]
fn parses_modmain_map_save() {
    let decompressed = try_gunzip_buffer(MODMAIN_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 417);
    assert_eq!(header.global_variable_count, 4);

    assert_eq!(variables.local_variables.len(), 417);
    assert_eq!(variables.global_variables.len(), 4);

    assert_eq!(scripts.len(), 55);
}

#[test]
fn parses_ncr_map_entrance_map_save() {
    let decompressed = try_gunzip_buffer(NCRENT_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 634);
    assert_eq!(header.global_variable_count, 7);

    assert_eq!(variables.local_variables.len(), 634);
    assert_eq!(variables.global_variables.len(), 7);

    assert_eq!(scripts.len(), 84);
}

#[test]
fn parses_newr1_map_save() {
    let decompressed = try_gunzip_buffer(NEWR1_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 858);
    assert_eq!(header.global_variable_count, 1);

    assert_eq!(variables.local_variables.len(), 858);
    assert_eq!(variables.global_variables.len(), 1);

    assert_eq!(scripts.len(), 191);
}

#[test]
fn parses_newr2_map_save() {
    let decompressed = try_gunzip_buffer(NEWR2_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 949);
    assert_eq!(header.global_variable_count, 1);

    assert_eq!(variables.local_variables.len(), 949);
    assert_eq!(variables.global_variables.len(), 1);

    assert_eq!(scripts.len(), 215);
}

#[test]
fn parses_newr3_map_save() {
    let decompressed = try_gunzip_buffer(NEWR3_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 221);
    assert_eq!(header.global_variable_count, 0);

    assert_eq!(variables.local_variables.len(), 221);
    assert_eq!(variables.global_variables.len(), 0);

    assert_eq!(scripts.len(), 56);
}

#[test]
fn parses_newrst_map_save() {
    let decompressed = try_gunzip_buffer(NEWRST_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 319);
    assert_eq!(header.global_variable_count, 2);

    assert_eq!(variables.local_variables.len(), 319);
    assert_eq!(variables.global_variables.len(), 2);

    assert_eq!(scripts.len(), 73);
}

#[test]
fn parses_raiders_map_2_map_save() {
    let decompressed = try_gunzip_buffer(RAIDERS2_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 337);
    assert_eq!(header.global_variable_count, 3);

    assert_eq!(variables.local_variables.len(), 337);
    assert_eq!(variables.global_variables.len(), 3);

    assert_eq!(scripts.len(), 177);
}

#[test]
fn parses_denbus2_map_save() {
    let decompressed = try_gunzip_buffer(REDDOWN_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 757);
    assert_eq!(header.global_variable_count, 5);

    assert_eq!(variables.local_variables.len(), 757);
    assert_eq!(variables.global_variables.len(), 5);

    assert_eq!(scripts.len(), 96);
}

#[test]
fn parses_redding_mine_entrance_map_save() {
    let decompressed = try_gunzip_buffer(REDMENT_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 808);
    assert_eq!(header.global_variable_count, 11);

    assert_eq!(variables.local_variables.len(), 808);
    assert_eq!(variables.global_variables.len(), 11);

    assert_eq!(scripts.len(), 94);
}

#[test]
fn parses_arroyo_caves_map_save() {
    let decompressed = try_gunzip_buffer(REDMTUN_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 6);
    assert_eq!(header.global_variable_count, 0);

    assert_eq!(variables.local_variables.len(), 6);
    assert_eq!(variables.global_variables.len(), 0);

    assert_eq!(scripts.len(), 13);
}

#[test]
fn parses_arroyo_caves_2_map_save() {
    let decompressed = try_gunzip_buffer(REDWAME_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 185);
    assert_eq!(header.global_variable_count, 16);

    assert_eq!(variables.local_variables.len(), 185);
    assert_eq!(variables.global_variables.len(), 16);

    assert_eq!(scripts.len(), 35);
}

#[test]
fn parses_v15ent_map_save() {
    let decompressed = try_gunzip_buffer(V15ENT_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 106);
    assert_eq!(header.global_variable_count, 2);

    assert_eq!(variables.local_variables.len(), 106);
    assert_eq!(variables.global_variables.len(), 2);

    assert_eq!(scripts.len(), 18);
}

#[test]
fn parses_vault15_secret_entrance_map_map_save() {
    let decompressed = try_gunzip_buffer(V15SENT_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 60);
    assert_eq!(header.global_variable_count, 1);

    assert_eq!(variables.local_variables.len(), 60);
    assert_eq!(variables.global_variables.len(), 1);

    assert_eq!(scripts.len(), 7);
}

#[test]
fn parses_arroyo_bridge_2_map_save() {
    let decompressed = try_gunzip_buffer(VCTYCOCL_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 400);
    assert_eq!(header.global_variable_count, 1);

    assert_eq!(variables.local_variables.len(), 400);
    assert_eq!(variables.global_variables.len(), 1);

    assert_eq!(scripts.len(), 52);
}

#[test]
fn parses_vctyctyd_map_save() {
    let decompressed = try_gunzip_buffer(VCTYCTYD_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 277);
    assert_eq!(header.global_variable_count, 7);

    assert_eq!(variables.local_variables.len(), 277);
    assert_eq!(variables.global_variables.len(), 7);

    assert_eq!(scripts.len(), 49);
}

#[test]
fn parses_arroyo_bridge_3_map_save() {
    let decompressed = try_gunzip_buffer(VCTYDWTN_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 460);
    assert_eq!(header.global_variable_count, 9);

    assert_eq!(variables.local_variables.len(), 460);
    assert_eq!(variables.global_variables.len(), 9);

    assert_eq!(scripts.len(), 74);
}

#[test]
fn parses_vault_city_vault_map_save() {
    let decompressed = try_gunzip_buffer(VCTYVLT_SAVE.to_vec());
    let (header, variables, scripts) = map_save(&decompressed);

    assert_eq!(header.local_variable_count, 87);
    assert_eq!(header.global_variable_count, 5);

    assert_eq!(variables.local_variables.len(), 87);
    assert_eq!(variables.global_variables.len(), 5);

    assert_eq!(scripts.len(), 25);
}

#[test]
fn parses_arroyo_bridge_map_save_scripts() {
    let decompressed = try_gunzip_buffer(ARBRIDGE_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 9);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 7);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 7);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_arroyo_caves_map_save_scripts() {
    let decompressed = try_gunzip_buffer(ARCAVES_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 30);
    assert_eq!(script.local_variable_offset, 42);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 31);
    assert_eq!(script.local_variable_offset, 74);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 32);
    assert_eq!(script.local_variable_offset, 98);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 33);
    assert_eq!(script.local_variable_offset, 122);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 172);
    assert_eq!(script.local_variable_offset, 50);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 173);
    assert_eq!(script.local_variable_offset, 58);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 175);
    assert_eq!(script.local_variable_offset, 66);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 176);
    assert_eq!(script.local_variable_offset, 82);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 177);
    assert_eq!(script.local_variable_offset, 90);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 178);
    assert_eq!(script.local_variable_offset, 106);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 179);
    assert_eq!(script.local_variable_offset, 154);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 180);
    assert_eq!(script.local_variable_offset, 162);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 181);
    assert_eq!(script.local_variable_offset, 130);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 182);
    assert_eq!(script.local_variable_offset, 138);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 183);
    assert_eq!(script.local_variable_offset, 114);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 186);
    assert_eq!(script.local_variable_offset, 170);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 174);
    assert_eq!(script.local_variable_offset, 146);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 184);
    assert_eq!(script.local_variable_offset, 178);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 25);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 758);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 749);
    assert_eq!(script.local_variable_offset, 8);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 749);
    assert_eq!(script.local_variable_offset, 17);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 19);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 748);
    assert_eq!(script.local_variable_offset, 26);
    assert_eq!(script.local_variable_count, 16);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_arroyo_village_map_save_scripts() {
    let decompressed = try_gunzip_buffer(ARGARDEN_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 266);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 266);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 266);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 266);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 266);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 266);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 266);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 266);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 332);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());
}

#[test]
fn parses_arroyo_caves_map_1_save_scripts() {
    let decompressed = try_gunzip_buffer(ARTEMPLE_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 750);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_arroyo_village_map_2_save_scripts() {
    let decompressed = try_gunzip_buffer(ARVILLAG_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 157);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 216);
    assert_eq!(script.local_variable_offset, 283);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 265);
    assert_eq!(script.local_variable_offset, 158);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 166);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 167);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 168);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 169);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 170);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 171);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 172);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 173);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 174);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 175);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 176);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 177);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 178);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 179);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 180);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 181);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 182);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 183);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 184);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 185);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 186);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 187);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 188);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 189);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 190);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 191);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 192);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 193);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 194);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 195);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 196);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 197);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 198);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 199);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 200);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 201);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 202);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 203);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 204);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 205);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 206);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 207);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 208);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 209);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 210);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 211);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 212);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 213);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 214);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 215);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 216);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 217);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 218);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 219);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 220);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 221);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 222);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 223);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 224);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 225);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 226);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 227);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 228);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 229);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 230);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 231);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 232);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 233);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 234);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 235);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 236);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 237);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 238);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 239);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 240);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 241);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 242);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 243);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 244);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 245);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 246);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[85];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 247);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[86];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 248);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[87];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 249);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[88];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 250);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[89];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 251);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[90];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 252);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[91];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 253);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[92];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 254);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[93];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 255);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[94];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 256);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[95];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 257);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[96];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 258);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[97];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 259);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[98];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 260);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[99];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 261);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[100];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 262);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[101];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 263);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[102];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 264);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[103];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 265);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[104];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 266);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[105];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 267);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[106];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 268);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[107];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 269);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[108];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 270);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[109];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 271);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[110];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 272);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[111];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 273);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[112];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 274);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[113];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 275);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[114];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 276);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[115];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 277);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[116];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 278);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[117];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 279);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[118];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 280);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[119];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 281);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[120];

    assert_eq!(script.id, 751);
    assert_eq!(script.local_variable_offset, 282);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[121];

    assert_eq!(script.id, 11);
    assert_eq!(script.local_variable_offset, 137);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[122];

    assert_eq!(script.id, 10);
    assert_eq!(script.local_variable_offset, 147);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[123];

    assert_eq!(script.id, 16);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[124];

    assert_eq!(script.id, 16);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[125];

    assert_eq!(script.id, 16);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[126];

    assert_eq!(script.id, 16);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[127];

    assert_eq!(script.id, 8);
    assert_eq!(script.local_variable_offset, 1);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[128];

    assert_eq!(script.id, 215);
    assert_eq!(script.local_variable_offset, 11);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[129];

    assert_eq!(script.id, 220);
    assert_eq!(script.local_variable_offset, 21);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[130];

    assert_eq!(script.id, 7);
    assert_eq!(script.local_variable_offset, 31);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[131];

    assert_eq!(script.id, 7);
    assert_eq!(script.local_variable_offset, 41);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[132];

    assert_eq!(script.id, 8);
    assert_eq!(script.local_variable_offset, 51);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[133];

    assert_eq!(script.id, 8);
    assert_eq!(script.local_variable_offset, 61);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[134];

    assert_eq!(script.id, 8);
    assert_eq!(script.local_variable_offset, 71);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[135];

    assert_eq!(script.id, 8);
    assert_eq!(script.local_variable_offset, 81);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[136];

    assert_eq!(script.id, 748);
    assert_eq!(script.local_variable_offset, 121);
    assert_eq!(script.local_variable_count, 16);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[137];

    assert_eq!(script.id, 212);
    assert_eq!(script.local_variable_offset, 111);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[138];

    assert_eq!(script.id, 213);
    assert_eq!(script.local_variable_offset, 91);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[139];

    assert_eq!(script.id, 214);
    assert_eq!(script.local_variable_offset, 101);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[140];

    assert_eq!(script.id, 221);
    assert_eq!(script.local_variable_offset, 286);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_broken_hills_village_map_save_scripts() {
    let decompressed = try_gunzip_buffer(BROKEN1_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 1189);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 889);
    assert_eq!(script.local_variable_offset, 2);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 30);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 40);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 50);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 60);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 70);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 80);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 90);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 100);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 110);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 120);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 130);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 140);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 150);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 160);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 170);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 180);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 190);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 200);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 210);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 220);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 230);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 240);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 250);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 260);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 270);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 280);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 290);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 300);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 310);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 320);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 330);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 340);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 350);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 360);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 370);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 380);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 390);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 400);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 410);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 420);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 430);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 440);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 450);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 460);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 889);
    assert_eq!(script.local_variable_offset, 470);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 666);
    assert_eq!(script.local_variable_offset, 478);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 992);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 994);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 1080);
    assert_eq!(script.local_variable_offset, 479);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 1081);
    assert_eq!(script.local_variable_offset, 480);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 1082);
    assert_eq!(script.local_variable_offset, 481);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 1133);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 1132);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 1272);
    assert_eq!(script.local_variable_offset, 482);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 604);
    assert_eq!(script.local_variable_offset, 548);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 1190);
    assert_eq!(script.local_variable_offset, 563);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 1193);
    assert_eq!(script.local_variable_offset, 571);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 1178);
    assert_eq!(script.local_variable_offset, 486);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 1193);
    assert_eq!(script.local_variable_offset, 579);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 1193);
    assert_eq!(script.local_variable_offset, 587);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 1193);
    assert_eq!(script.local_variable_offset, 595);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 1193);
    assert_eq!(script.local_variable_offset, 603);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 1193);
    assert_eq!(script.local_variable_offset, 611);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 1193);
    assert_eq!(script.local_variable_offset, 619);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 1193);
    assert_eq!(script.local_variable_offset, 627);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 1193);
    assert_eq!(script.local_variable_offset, 635);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 589);
    assert_eq!(script.local_variable_offset, 643);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 595);
    assert_eq!(script.local_variable_offset, 653);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 595);
    assert_eq!(script.local_variable_offset, 663);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 595);
    assert_eq!(script.local_variable_offset, 673);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 595);
    assert_eq!(script.local_variable_offset, 683);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 605);
    assert_eq!(script.local_variable_offset, 693);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 607);
    assert_eq!(script.local_variable_offset, 705);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 1193);
    assert_eq!(script.local_variable_offset, 720);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 612);
    assert_eq!(script.local_variable_offset, 728);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 1159);
    assert_eq!(script.local_variable_offset, 738);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 1149);
    assert_eq!(script.local_variable_offset, 495);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 1159);
    assert_eq!(script.local_variable_offset, 751);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 1159);
    assert_eq!(script.local_variable_offset, 764);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 594);
    assert_eq!(script.local_variable_offset, 777);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 598);
    assert_eq!(script.local_variable_offset, 788);
    assert_eq!(script.local_variable_count, 16);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[85];

    assert_eq!(script.id, 603);
    assert_eq!(script.local_variable_offset, 804);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[86];

    assert_eq!(script.id, 593);
    assert_eq!(script.local_variable_offset, 816);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[87];

    assert_eq!(script.id, 588);
    assert_eq!(script.local_variable_offset, 827);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[88];

    assert_eq!(script.id, 599);
    assert_eq!(script.local_variable_offset, 888);
    assert_eq!(script.local_variable_count, 20);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[89];

    assert_eq!(script.id, 587);
    assert_eq!(script.local_variable_offset, 840);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[90];

    assert_eq!(script.id, 1193);
    assert_eq!(script.local_variable_offset, 850);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[91];

    assert_eq!(script.id, 592);
    assert_eq!(script.local_variable_offset, 858);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[92];

    assert_eq!(script.id, 597);
    assert_eq!(script.local_variable_offset, 868);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[93];

    assert_eq!(script.id, 1131);
    assert_eq!(script.local_variable_offset, 511);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[94];

    assert_eq!(script.id, 1149);
    assert_eq!(script.local_variable_offset, 503);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[95];

    assert_eq!(script.id, 596);
    assert_eq!(script.local_variable_offset, 518);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[96];

    assert_eq!(script.id, 1194);
    assert_eq!(script.local_variable_offset, 530);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[97];

    assert_eq!(script.id, 602);
    assert_eq!(script.local_variable_offset, 538);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_broken_hills_village_map_1_save_scripts() {
    let decompressed = try_gunzip_buffer(BROKEN2_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 876);
    assert_eq!(script.local_variable_offset, 514);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 1137);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 1140);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 1140);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 1165);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 873);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 30);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 40);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 50);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 60);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 70);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 80);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 90);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 100);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 110);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 1175);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 120);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 130);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 140);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 150);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 160);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 170);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 180);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 190);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 200);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 210);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 220);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 230);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 240);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 250);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 260);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 270);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 280);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 290);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 300);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 1068);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 1174);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 606);
    assert_eq!(script.local_variable_offset, 310);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 600);
    assert_eq!(script.local_variable_offset, 474);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 600);
    assert_eq!(script.local_variable_offset, 484);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 600);
    assert_eq!(script.local_variable_offset, 494);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 600);
    assert_eq!(script.local_variable_offset, 504);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 600);
    assert_eq!(script.local_variable_offset, 320);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 1079);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 1117);
    assert_eq!(script.local_variable_offset, 330);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 601);
    assert_eq!(script.local_variable_offset, 339);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 601);
    assert_eq!(script.local_variable_offset, 349);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 592);
    assert_eq!(script.local_variable_offset, 359);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 597);
    assert_eq!(script.local_variable_offset, 369);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 597);
    assert_eq!(script.local_variable_offset, 379);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 592);
    assert_eq!(script.local_variable_offset, 389);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 1139);
    assert_eq!(script.local_variable_offset, 399);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[85];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[86];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[87];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[88];

    assert_eq!(script.id, 1135);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[89];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[90];

    assert_eq!(script.id, 1192);
    assert_eq!(script.local_variable_offset, 466);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[91];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[92];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[93];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[94];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[95];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[96];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[97];

    assert_eq!(script.id, 1164);
    assert_eq!(script.local_variable_offset, 407);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[98];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[99];

    assert_eq!(script.id, 1192);
    assert_eq!(script.local_variable_offset, 458);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[100];

    assert_eq!(script.id, 1192);
    assert_eq!(script.local_variable_offset, 450);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[101];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[102];

    assert_eq!(script.id, 1135);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[103];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[104];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[105];

    assert_eq!(script.id, 1135);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[106];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[107];

    assert_eq!(script.id, 1176);
    assert_eq!(script.local_variable_offset, 415);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[108];

    assert_eq!(script.id, 1185);
    assert_eq!(script.local_variable_offset, 438);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[109];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[110];

    assert_eq!(script.id, 1173);
    assert_eq!(script.local_variable_offset, 424);
    assert_eq!(script.local_variable_count, 14);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[111];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[112];

    assert_eq!(script.id, 1135);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[113];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[114];

    assert_eq!(script.id, 1135);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[115];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[116];

    assert_eq!(script.id, 1135);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[117];

    assert_eq!(script.id, 1134);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[118];

    assert_eq!(script.id, 1135);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[119];

    assert_eq!(script.id, 1135);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_denbus1_map_save_scripts() {
    let decompressed = try_gunzip_buffer(DENBUS1_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 917);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 67);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 18);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 26);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 34);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 42);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 50);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 58);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 66);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 74);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 82);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 90);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 537);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 918);
    assert_eq!(script.local_variable_offset, 98);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 912);
    assert_eq!(script.local_variable_offset, 723);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 169);
    assert_eq!(script.local_variable_offset, 106);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 114);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 1158);
    assert_eq!(script.local_variable_offset, 591);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 916);
    assert_eq!(script.local_variable_offset, 124);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 984);
    assert_eq!(script.local_variable_offset, 132);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 669);
    assert_eq!(script.local_variable_offset, 138);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 303);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 919);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 47);
    assert_eq!(script.local_variable_offset, 146);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 41);
    assert_eq!(script.local_variable_offset, 766);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 39);
    assert_eq!(script.local_variable_offset, 156);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 39);
    assert_eq!(script.local_variable_offset, 167);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 39);
    assert_eq!(script.local_variable_offset, 178);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 39);
    assert_eq!(script.local_variable_offset, 189);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 51);
    assert_eq!(script.local_variable_offset, 200);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 915);
    assert_eq!(script.local_variable_offset, 781);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 212);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 257);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 264);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 904);
    assert_eq!(script.local_variable_offset, 642);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 44);
    assert_eq!(script.local_variable_offset, 271);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 904);
    assert_eq!(script.local_variable_offset, 278);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 904);
    assert_eq!(script.local_variable_offset, 286);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 294);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 301);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 308);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 74);
    assert_eq!(script.local_variable_offset, 594);
    assert_eq!(script.local_variable_count, 43);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 731);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 905);
    assert_eq!(script.local_variable_offset, 315);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 905);
    assert_eq!(script.local_variable_offset, 323);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 331);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 338);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 904);
    assert_eq!(script.local_variable_offset, 650);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 345);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 352);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 359);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 905);
    assert_eq!(script.local_variable_offset, 366);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 374);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 381);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 388);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 36);
    assert_eq!(script.local_variable_offset, 219);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 395);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 36);
    assert_eq!(script.local_variable_offset, 227);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 402);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 409);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 416);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 423);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 430);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 437);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 444);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 451);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 37);
    assert_eq!(script.local_variable_offset, 458);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 37);
    assert_eq!(script.local_variable_offset, 470);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 37);
    assert_eq!(script.local_variable_offset, 482);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 37);
    assert_eq!(script.local_variable_offset, 494);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 905);
    assert_eq!(script.local_variable_offset, 506);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 905);
    assert_eq!(script.local_variable_offset, 514);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 905);
    assert_eq!(script.local_variable_offset, 522);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 48);
    assert_eq!(script.local_variable_offset, 235);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 37);
    assert_eq!(script.local_variable_offset, 530);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 38);
    assert_eq!(script.local_variable_offset, 758);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 40);
    assert_eq!(script.local_variable_offset, 542);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 554);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 561);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 568);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 36);
    assert_eq!(script.local_variable_offset, 241);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 909);
    assert_eq!(script.local_variable_offset, 249);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 74);
    assert_eq!(script.local_variable_offset, 658);
    assert_eq!(script.local_variable_count, 43);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 575);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[85];

    assert_eq!(script.id, 983);
    assert_eq!(script.local_variable_offset, 582);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[86];

    assert_eq!(script.id, 982);
    assert_eq!(script.local_variable_offset, 750);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[87];

    assert_eq!(script.id, 982);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[88];

    assert_eq!(script.id, 1155);
    assert_eq!(script.local_variable_offset, 701);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[89];

    assert_eq!(script.id, 940);
    assert_eq!(script.local_variable_offset, 736);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[90];

    assert_eq!(script.id, 38);
    assert_eq!(script.local_variable_offset, 742);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[91];

    assert_eq!(script.id, 1263);
    assert_eq!(script.local_variable_offset, 710);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[92];

    assert_eq!(script.id, 1296);
    assert_eq!(script.local_variable_offset, 756);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[93];

    assert_eq!(script.id, 1296);
    assert_eq!(script.local_variable_offset, 757);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[94];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 718);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_denbus2_map_save_scripts() {
    let decompressed = try_gunzip_buffer(DENBUS2_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 8);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 13);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 973);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 18);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 23);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 950);
    assert_eq!(script.local_variable_offset, 28);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 36);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 41);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 62);
    assert_eq!(script.local_variable_offset, 46);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 51);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 56);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 61);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 66);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 71);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 76);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 61);
    assert_eq!(script.local_variable_offset, 81);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 86);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 91);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 96);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 101);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 106);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 111);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 116);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 121);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 126);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 131);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 136);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 63);
    assert_eq!(script.local_variable_offset, 141);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 146);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 151);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 156);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 161);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 169);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 177);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 185);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 193);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 201);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 209);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 72);
    assert_eq!(script.local_variable_offset, 217);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 61);
    assert_eq!(script.local_variable_offset, 225);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 230);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 930);
    assert_eq!(script.local_variable_offset, 238);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 537);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 239);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 247);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 255);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 917);
    assert_eq!(script.local_variable_offset, 263);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 951);
    assert_eq!(script.local_variable_offset, 271);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 945);
    assert_eq!(script.local_variable_offset, 279);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 287);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 514);
    assert_eq!(script.local_variable_offset, 295);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 973);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 973);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 973);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 973);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 973);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 45);
    assert_eq!(script.local_variable_offset, 303);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 313);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 42);
    assert_eq!(script.local_variable_offset, 537);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 42);
    assert_eq!(script.local_variable_offset, 544);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 42);
    assert_eq!(script.local_variable_offset, 551);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 39);
    assert_eq!(script.local_variable_offset, 320);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 39);
    assert_eq!(script.local_variable_offset, 331);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 46);
    assert_eq!(script.local_variable_offset, 342);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 910);
    assert_eq!(script.local_variable_offset, 351);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 52);
    assert_eq!(script.local_variable_offset, 359);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 53);
    assert_eq!(script.local_variable_offset, 371);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 54);
    assert_eq!(script.local_variable_offset, 826);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 914);
    assert_eq!(script.local_variable_offset, 378);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 908);
    assert_eq!(script.local_variable_offset, 388);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 909);
    assert_eq!(script.local_variable_offset, 558);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 46);
    assert_eq!(script.local_variable_offset, 398);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 46);
    assert_eq!(script.local_variable_offset, 407);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 46);
    assert_eq!(script.local_variable_offset, 416);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 907);
    assert_eq!(script.local_variable_offset, 425);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 36);
    assert_eq!(script.local_variable_offset, 566);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 435);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 442);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 37);
    assert_eq!(script.local_variable_offset, 449);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 37);
    assert_eq!(script.local_variable_offset, 461);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 42);
    assert_eq!(script.local_variable_offset, 574);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 36);
    assert_eq!(script.local_variable_offset, 581);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 37);
    assert_eq!(script.local_variable_offset, 473);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 36);
    assert_eq!(script.local_variable_offset, 589);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[85];

    assert_eq!(script.id, 103);
    assert_eq!(script.local_variable_offset, 791);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[86];

    assert_eq!(script.id, 37);
    assert_eq!(script.local_variable_offset, 485);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[87];

    assert_eq!(script.id, 37);
    assert_eq!(script.local_variable_offset, 497);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[88];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 509);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[89];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 516);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[90];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 523);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[91];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 530);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[92];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 605);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[93];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 612);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[94];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 619);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[95];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 626);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[96];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 633);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[97];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 640);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[98];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 647);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[99];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 654);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[100];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 661);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[101];

    assert_eq!(script.id, 911);
    assert_eq!(script.local_variable_offset, 597);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[102];

    assert_eq!(script.id, 907);
    assert_eq!(script.local_variable_offset, 668);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[103];

    assert_eq!(script.id, 908);
    assert_eq!(script.local_variable_offset, 678);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[104];

    assert_eq!(script.id, 913);
    assert_eq!(script.local_variable_offset, 688);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[105];

    assert_eq!(script.id, 49);
    assert_eq!(script.local_variable_offset, 698);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[106];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 710);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[107];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 717);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[108];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 724);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[109];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 731);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[110];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 738);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[111];

    assert_eq!(script.id, 35);
    assert_eq!(script.local_variable_offset, 745);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[112];

    assert_eq!(script.id, 903);
    assert_eq!(script.local_variable_offset, 801);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[113];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[114];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[115];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[116];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[117];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[118];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[119];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[120];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[121];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[122];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[123];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[124];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[125];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[126];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[127];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[128];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[129];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[130];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[131];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[132];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[133];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[134];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[135];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[136];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[137];

    assert_eq!(script.id, 906);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[138];

    assert_eq!(script.id, 46);
    assert_eq!(script.local_variable_offset, 752);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[139];

    assert_eq!(script.id, 46);
    assert_eq!(script.local_variable_offset, 761);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[140];

    assert_eq!(script.id, 46);
    assert_eq!(script.local_variable_offset, 770);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[141];

    assert_eq!(script.id, 50);
    assert_eq!(script.local_variable_offset, 779);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[142];

    assert_eq!(script.id, 941);
    assert_eq!(script.local_variable_offset, 819);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[143];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 814);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[144];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 843);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_gecko_junkyard_map_map_save_scripts() {
    let decompressed = try_gunzip_buffer(GECKJUNK_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 30);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 40);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 50);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 60);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 70);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 80);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 90);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 100);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 110);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 120);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 1260);
    assert_eq!(script.local_variable_offset, 130);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 135);
    assert_eq!(script.local_variable_offset, 133);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 134);
    assert_eq!(script.local_variable_offset, 145);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 191);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 199);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 131);
    assert_eq!(script.local_variable_offset, 157);
    assert_eq!(script.local_variable_count, 16);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 754);
    assert_eq!(script.local_variable_offset, 173);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 1259);
    assert_eq!(script.local_variable_offset, 181);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_gecko_power_plant_map_script_map_save_scripts() {
    let decompressed = try_gunzip_buffer(GECKPWPL_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 464);
    assert_eq!(script.local_variable_offset, 25);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 463);
    assert_eq!(script.local_variable_offset, 34);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 515);
    assert_eq!(script.local_variable_offset, 285);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 529);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 529);
    assert_eq!(script.local_variable_offset, 5);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 529);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 529);
    assert_eq!(script.local_variable_offset, 15);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 529);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 846);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 43);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 46);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 49);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 52);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 55);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 58);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 61);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 64);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 67);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 70);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 73);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 76);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 79);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 82);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 85);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 999);
    assert_eq!(script.local_variable_offset, 88);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 132);
    assert_eq!(script.local_variable_offset, 172);
    assert_eq!(script.local_variable_count, 16);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 130);
    assert_eq!(script.local_variable_offset, 264);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 133);
    assert_eq!(script.local_variable_offset, 188);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 139);
    assert_eq!(script.local_variable_offset, 218);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 404);
    assert_eq!(script.local_variable_offset, 298);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 403);
    assert_eq!(script.local_variable_offset, 237);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 403);
    assert_eq!(script.local_variable_offset, 246);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 142);
    assert_eq!(script.local_variable_offset, 276);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 392);
    assert_eq!(script.local_variable_offset, 200);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 141);
    assert_eq!(script.local_variable_offset, 209);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 759);
    assert_eq!(script.local_variable_offset, 91);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 759);
    assert_eq!(script.local_variable_offset, 100);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 141);
    assert_eq!(script.local_variable_offset, 109);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 392);
    assert_eq!(script.local_variable_offset, 118);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 404);
    assert_eq!(script.local_variable_offset, 228);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 404);
    assert_eq!(script.local_variable_offset, 255);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 395);
    assert_eq!(script.local_variable_offset, 290);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 392);
    assert_eq!(script.local_variable_offset, 127);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 392);
    assert_eq!(script.local_variable_offset, 136);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 759);
    assert_eq!(script.local_variable_offset, 145);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 759);
    assert_eq!(script.local_variable_offset, 154);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 759);
    assert_eq!(script.local_variable_offset, 163);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 317);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 322);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_gecko_settlement_map_map_save_scripts() {
    let decompressed = try_gunzip_buffer(GECKSETL_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 30);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 40);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 50);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 60);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 70);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 80);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 137);
    assert_eq!(script.local_variable_offset, 90);
    assert_eq!(script.local_variable_count, 19);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 279);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 271);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 263);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 255);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 215);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 247);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 398);
    assert_eq!(script.local_variable_offset, 109);
    assert_eq!(script.local_variable_count, 20);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 754);
    assert_eq!(script.local_variable_offset, 129);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 287);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 223);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 311);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 295);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 319);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 303);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 231);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 144);
    assert_eq!(script.local_variable_offset, 239);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 754);
    assert_eq!(script.local_variable_offset, 137);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 754);
    assert_eq!(script.local_variable_offset, 145);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 754);
    assert_eq!(script.local_variable_offset, 153);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 754);
    assert_eq!(script.local_variable_offset, 161);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 754);
    assert_eq!(script.local_variable_offset, 169);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 754);
    assert_eq!(script.local_variable_offset, 177);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 138);
    assert_eq!(script.local_variable_offset, 185);
    assert_eq!(script.local_variable_count, 20);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 612);
    assert_eq!(script.local_variable_offset, 205);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_gecko_tunnel_map_map_save_scripts() {
    let decompressed = try_gunzip_buffer(GECKTUNL_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 136);
    assert_eq!(script.local_variable_offset, 80);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 397);
    assert_eq!(script.local_variable_offset, 72);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 397);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 397);
    assert_eq!(script.local_variable_offset, 8);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 397);
    assert_eq!(script.local_variable_offset, 16);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 397);
    assert_eq!(script.local_variable_offset, 24);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 397);
    assert_eq!(script.local_variable_offset, 32);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 397);
    assert_eq!(script.local_variable_offset, 40);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 397);
    assert_eq!(script.local_variable_offset, 48);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 397);
    assert_eq!(script.local_variable_offset, 56);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 397);
    assert_eq!(script.local_variable_offset, 64);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 129);
    assert_eq!(script.local_variable_offset, 91);
    assert_eq!(script.local_variable_count, 18);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_gstcav1_map_save_scripts() {
    let decompressed = try_gunzip_buffer(GSTCAV1_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 579);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 573);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 573);
    assert_eq!(script.local_variable_offset, 8);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 573);
    assert_eq!(script.local_variable_offset, 12);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_gstcav2_map_save_scripts() {
    let decompressed = try_gunzip_buffer(GSTCAV2_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 102);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 586);
    assert_eq!(script.local_variable_offset, 53);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 586);
    assert_eq!(script.local_variable_offset, 13);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 586);
    assert_eq!(script.local_variable_offset, 21);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 586);
    assert_eq!(script.local_variable_offset, 29);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 586);
    assert_eq!(script.local_variable_offset, 37);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 586);
    assert_eq!(script.local_variable_offset, 45);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_gstfarm_map_save_scripts() {
    let decompressed = try_gunzip_buffer(GSTFARM_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 585);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 665);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 665);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 609);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 15);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_klacanyn_map_save_scripts() {
    let decompressed = try_gunzip_buffer(KLACANYN_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 85);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 20);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_klamath_village_map_save_scripts() {
    let decompressed = try_gunzip_buffer(KLADWTWN_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 331);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 739);
    assert_eq!(script.local_variable_offset, 670);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 264);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 315);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 316);
    assert_eq!(script.local_variable_offset, 8);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 317);
    assert_eq!(script.local_variable_offset, 16);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 330);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 267);
    assert_eq!(script.local_variable_offset, 24);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 32);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 42);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 52);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 62);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 72);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 82);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 92);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 102);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 112);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 122);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 669);
    assert_eq!(script.local_variable_offset, 132);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 140);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 150);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 160);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 170);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 180);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 190);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 200);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 210);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 220);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 230);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 240);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 250);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 260);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 270);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 280);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 290);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 300);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 310);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 320);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 330);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 340);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 350);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 360);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 669);
    assert_eq!(script.local_variable_offset, 370);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 847);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 847);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 847);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 847);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 81);
    assert_eq!(script.local_variable_offset, 378);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 83);
    assert_eq!(script.local_variable_offset, 391);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 77);
    assert_eq!(script.local_variable_offset, 404);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 80);
    assert_eq!(script.local_variable_offset, 412);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 73);
    assert_eq!(script.local_variable_offset, 427);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 85);
    assert_eq!(script.local_variable_offset, 442);
    assert_eq!(script.local_variable_count, 20);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 75);
    assert_eq!(script.local_variable_offset, 462);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 296);
    assert_eq!(script.local_variable_offset, 657);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 95);
    assert_eq!(script.local_variable_offset, 473);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 78);
    assert_eq!(script.local_variable_offset, 483);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 77);
    assert_eq!(script.local_variable_offset, 491);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 78);
    assert_eq!(script.local_variable_offset, 499);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 78);
    assert_eq!(script.local_variable_offset, 507);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 78);
    assert_eq!(script.local_variable_offset, 515);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 77);
    assert_eq!(script.local_variable_offset, 523);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 78);
    assert_eq!(script.local_variable_offset, 531);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 77);
    assert_eq!(script.local_variable_offset, 539);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 76);
    assert_eq!(script.local_variable_offset, 547);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 77);
    assert_eq!(script.local_variable_offset, 562);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 77);
    assert_eq!(script.local_variable_offset, 570);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 297);
    assert_eq!(script.local_variable_offset, 578);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 297);
    assert_eq!(script.local_variable_offset, 586);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 79);
    assert_eq!(script.local_variable_offset, 594);
    assert_eq!(script.local_variable_count, 21);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 296);
    assert_eq!(script.local_variable_offset, 615);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 296);
    assert_eq!(script.local_variable_offset, 623);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 296);
    assert_eq!(script.local_variable_offset, 631);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 299);
    assert_eq!(script.local_variable_offset, 639);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 299);
    assert_eq!(script.local_variable_offset, 642);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 299);
    assert_eq!(script.local_variable_offset, 645);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 1177);
    assert_eq!(script.local_variable_offset, 648);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 82);
    assert_eq!(script.local_variable_offset, 672);
    assert_eq!(script.local_variable_count, 18);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_klamath_graze_map_map_save_scripts() {
    let decompressed = try_gunzip_buffer(KLAGRAZ_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 85);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 20);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 302);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 302);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 302);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 302);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_arroyo_bridge_map_1_save_scripts() {
    let decompressed = try_gunzip_buffer(KLATOXCV_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 119);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 167);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 167);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 1200);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 1201);
    assert_eq!(script.local_variable_offset, 18);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 82);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 18);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 28);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 269);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 269);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 1235);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 269);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_klatrap_map_save_scripts() {
    let decompressed = try_gunzip_buffer(KLATRAP_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 295);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 269);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_modgard_map_save_scripts() {
    let decompressed = try_gunzip_buffer(MODGARD_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);
}

#[test]
fn parses_modinn_map_save_scripts() {
    let decompressed = try_gunzip_buffer(MODINN_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 206);
    assert_eq!(script.local_variable_offset, 253);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 8);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 210);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 211);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 208);
    assert_eq!(script.local_variable_offset, 16);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 17);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 25);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 33);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 41);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 49);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 57);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 65);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 73);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 81);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 107);
    assert_eq!(script.local_variable_offset, 233);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 97);
    assert_eq!(script.local_variable_offset, 89);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 99);
    assert_eq!(script.local_variable_offset, 215);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 816);
    assert_eq!(script.local_variable_offset, 207);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 815);
    assert_eq!(script.local_variable_offset, 199);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 815);
    assert_eq!(script.local_variable_offset, 191);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 815);
    assert_eq!(script.local_variable_offset, 183);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 561);
    assert_eq!(script.local_variable_offset, 246);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 102);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 107);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 112);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 117);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 122);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 814);
    assert_eq!(script.local_variable_offset, 225);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 815);
    assert_eq!(script.local_variable_offset, 127);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 815);
    assert_eq!(script.local_variable_offset, 135);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 815);
    assert_eq!(script.local_variable_offset, 143);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 815);
    assert_eq!(script.local_variable_offset, 151);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 816);
    assert_eq!(script.local_variable_offset, 159);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 816);
    assert_eq!(script.local_variable_offset, 167);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 816);
    assert_eq!(script.local_variable_offset, 175);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_modmain_map_save_scripts() {
    let decompressed = try_gunzip_buffer(MODMAIN_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 537);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 547);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 539);
    assert_eq!(script.local_variable_offset, 11);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 12);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 560);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 540);
    assert_eq!(script.local_variable_offset, 28);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 572);
    assert_eq!(script.local_variable_offset, 410);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 32);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 42);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 52);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 62);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 72);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 82);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 92);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 102);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 112);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 122);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 132);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 142);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 152);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 162);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 581);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 104);
    assert_eq!(script.local_variable_offset, 172);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 105);
    assert_eq!(script.local_variable_offset, 187);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 100);
    assert_eq!(script.local_variable_offset, 202);
    assert_eq!(script.local_variable_count, 14);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 98);
    assert_eq!(script.local_variable_offset, 216);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 96);
    assert_eq!(script.local_variable_offset, 229);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 577);
    assert_eq!(script.local_variable_offset, 237);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 553);
    assert_eq!(script.local_variable_offset, 240);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 345);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 355);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 350);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 385);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 380);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 375);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 370);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 365);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 575);
    assert_eq!(script.local_variable_offset, 360);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 101);
    assert_eq!(script.local_variable_offset, 252);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 580);
    assert_eq!(script.local_variable_offset, 267);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 816);
    assert_eq!(script.local_variable_offset, 275);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 816);
    assert_eq!(script.local_variable_offset, 283);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 815);
    assert_eq!(script.local_variable_offset, 291);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 816);
    assert_eq!(script.local_variable_offset, 299);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 816);
    assert_eq!(script.local_variable_offset, 307);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 203);
    assert_eq!(script.local_variable_offset, 315);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 103);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 330);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 390);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 335);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 340);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_ncr_map_1_map_save_scripts() {
    let decompressed = try_gunzip_buffer(NCR1_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 870);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 870);
    assert_eq!(script.local_variable_offset, 691);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 885);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 407);
    assert_eq!(script.local_variable_offset, 6);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 407);
    assert_eq!(script.local_variable_offset, 16);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 454);
    assert_eq!(script.local_variable_offset, 26);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 465);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 519);
    assert_eq!(script.local_variable_offset, 34);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 548);
    assert_eq!(script.local_variable_offset, 38);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 548);
    assert_eq!(script.local_variable_offset, 46);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 548);
    assert_eq!(script.local_variable_offset, 54);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 548);
    assert_eq!(script.local_variable_offset, 62);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 1158);
    assert_eq!(script.local_variable_offset, 688);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 850);
    assert_eq!(script.local_variable_offset, 70);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 850);
    assert_eq!(script.local_variable_offset, 76);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 548);
    assert_eq!(script.local_variable_offset, 82);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 863);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 90);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 885);
    assert_eq!(script.local_variable_offset, 100);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 106);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 116);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 1272);
    assert_eq!(script.local_variable_offset, 126);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 383);
    assert_eq!(script.local_variable_offset, 574);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 400);
    assert_eq!(script.local_variable_offset, 583);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 405);
    assert_eq!(script.local_variable_offset, 591);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 406);
    assert_eq!(script.local_variable_offset, 130);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 139);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 152);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 165);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 178);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 191);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 709);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 204);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 448);
    assert_eq!(script.local_variable_offset, 599);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 472);
    assert_eq!(script.local_variable_offset, 608);
    assert_eq!(script.local_variable_count, 14);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 453);
    assert_eq!(script.local_variable_offset, 622);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 456);
    assert_eq!(script.local_variable_offset, 630);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 471);
    assert_eq!(script.local_variable_offset, 640);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 466);
    assert_eq!(script.local_variable_offset, 649);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 462);
    assert_eq!(script.local_variable_offset, 658);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 474);
    assert_eq!(script.local_variable_offset, 217);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 449);
    assert_eq!(script.local_variable_offset, 226);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 513);
    assert_eq!(script.local_variable_offset, 668);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 401);
    assert_eq!(script.local_variable_offset, 678);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 234);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 582);
    assert_eq!(script.local_variable_offset, 522);
    assert_eq!(script.local_variable_count, 43);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 247);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 848);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 467);
    assert_eq!(script.local_variable_offset, 260);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 271);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 1154);
    assert_eq!(script.local_variable_offset, 565);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 284);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 294);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 304);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 314);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 324);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 334);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 344);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 354);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 364);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 374);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 384);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 394);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 404);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 414);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 424);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 1170);
    assert_eq!(script.local_variable_offset, 434);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 444);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 457);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 470);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 483);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 496);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 447);
    assert_eq!(script.local_variable_offset, 509);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 699);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 734);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 704);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 714);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 719);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 724);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 729);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_ncr_map_entrance_map_save_scripts() {
    let decompressed = try_gunzip_buffer(NCRENT_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 738);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 227);
    assert_eq!(script.local_variable_offset, 1);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 254);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 253);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 252);
    assert_eq!(script.local_variable_offset, 30);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 375);
    assert_eq!(script.local_variable_offset, 40);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 375);
    assert_eq!(script.local_variable_offset, 44);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 375);
    assert_eq!(script.local_variable_offset, 48);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 374);
    assert_eq!(script.local_variable_offset, 52);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 374);
    assert_eq!(script.local_variable_offset, 60);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 1184);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 866);
    assert_eq!(script.local_variable_offset, 602);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 863);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 375);
    assert_eq!(script.local_variable_offset, 68);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 374);
    assert_eq!(script.local_variable_offset, 72);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 374);
    assert_eq!(script.local_variable_offset, 80);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 335);
    assert_eq!(script.local_variable_offset, 88);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 96);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 104);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 112);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 120);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 128);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 136);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 144);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 152);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 160);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 168);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 176);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 184);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 311);
    assert_eq!(script.local_variable_offset, 192);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 864);
    assert_eq!(script.local_variable_offset, 200);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 864);
    assert_eq!(script.local_variable_offset, 206);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 1184);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 1184);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 1184);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 1202);
    assert_eq!(script.local_variable_offset, 212);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 1093);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 219);
    assert_eq!(script.local_variable_offset, 221);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 318);
    assert_eq!(script.local_variable_offset, 231);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 351);
    assert_eq!(script.local_variable_offset, 342);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 259);
    assert_eq!(script.local_variable_offset, 352);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 259);
    assert_eq!(script.local_variable_offset, 361);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 259);
    assert_eq!(script.local_variable_offset, 370);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 259);
    assert_eq!(script.local_variable_offset, 379);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 259);
    assert_eq!(script.local_variable_offset, 388);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 251);
    assert_eq!(script.local_variable_offset, 242);
    assert_eq!(script.local_variable_count, 14);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 397);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 405);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 413);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 421);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 429);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 437);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 445);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 453);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 461);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 469);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 477);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 485);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 300);
    assert_eq!(script.local_variable_offset, 493);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 324);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 298);
    assert_eq!(script.local_variable_offset, 256);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 373);
    assert_eq!(script.local_variable_offset, 501);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 379);
    assert_eq!(script.local_variable_offset, 265);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 370);
    assert_eq!(script.local_variable_offset, 274);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 371);
    assert_eq!(script.local_variable_offset, 509);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 373);
    assert_eq!(script.local_variable_offset, 517);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 334);
    assert_eq!(script.local_variable_offset, 525);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 226);
    assert_eq!(script.local_variable_offset, 283);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 379);
    assert_eq!(script.local_variable_offset, 293);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 365);
    assert_eq!(script.local_variable_offset, 533);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 612);
    assert_eq!(script.local_variable_offset, 541);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 691);
    assert_eq!(script.local_variable_offset, 551);
    assert_eq!(script.local_variable_count, 17);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 1182);
    assert_eq!(script.local_variable_offset, 302);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 1182);
    assert_eq!(script.local_variable_offset, 312);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 373);
    assert_eq!(script.local_variable_offset, 568);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 373);
    assert_eq!(script.local_variable_offset, 576);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 373);
    assert_eq!(script.local_variable_offset, 584);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 1182);
    assert_eq!(script.local_variable_offset, 322);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 1182);
    assert_eq!(script.local_variable_offset, 332);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 597);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 605);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 382);
    assert_eq!(script.local_variable_offset, 610);
    assert_eq!(script.local_variable_count, 24);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_newr1_map_save_scripts() {
    let decompressed = try_gunzip_buffer(NEWR1_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 667);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 725);
    assert_eq!(script.local_variable_offset, 614);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 726);
    assert_eq!(script.local_variable_offset, 615);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 711);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 1048);
    assert_eq!(script.local_variable_offset, 1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 7);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 17);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 27);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 33);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 39);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 45);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 51);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 57);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 63);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 69);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 75);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 81);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 87);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 459);
    assert_eq!(script.local_variable_offset, 93);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 461);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 94);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 104);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 114);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 124);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 134);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 144);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 154);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 164);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 174);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 184);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 194);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 204);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 214);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 224);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 234);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 244);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 254);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 264);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 274);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 284);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 294);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 304);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 1048);
    assert_eq!(script.local_variable_offset, 314);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 1048);
    assert_eq!(script.local_variable_offset, 320);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 1048);
    assert_eq!(script.local_variable_offset, 326);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 332);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 342);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 352);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 362);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 372);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 382);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 392);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 402);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 412);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 422);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 1218);
    assert_eq!(script.local_variable_offset, 836);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 1204);
    assert_eq!(script.local_variable_offset, 837);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 1204);
    assert_eq!(script.local_variable_offset, 838);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 1204);
    assert_eq!(script.local_variable_offset, 839);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 1204);
    assert_eq!(script.local_variable_offset, 840);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 1204);
    assert_eq!(script.local_variable_offset, 841);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 1204);
    assert_eq!(script.local_variable_offset, 842);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 1206);
    assert_eq!(script.local_variable_offset, 843);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 1223);
    assert_eq!(script.local_variable_offset, 844);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 1206);
    assert_eq!(script.local_variable_offset, 845);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 1206);
    assert_eq!(script.local_variable_offset, 846);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 1223);
    assert_eq!(script.local_variable_offset, 847);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 1206);
    assert_eq!(script.local_variable_offset, 848);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 1206);
    assert_eq!(script.local_variable_offset, 849);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 1205);
    assert_eq!(script.local_variable_offset, 850);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 1218);
    assert_eq!(script.local_variable_offset, 851);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 1218);
    assert_eq!(script.local_variable_offset, 852);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 1195);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 378);
    assert_eq!(script.local_variable_offset, 821);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 328);
    assert_eq!(script.local_variable_offset, 616);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 432);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 434);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 436);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 438);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 440);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[85];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 442);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[86];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 444);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[87];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 446);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[88];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 448);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[89];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 450);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[90];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 452);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[91];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 454);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[92];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 456);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[93];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 458);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[94];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 460);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[95];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 462);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[96];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 464);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[97];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 466);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[98];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 468);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[99];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 470);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[100];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 472);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[101];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 474);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[102];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 476);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[103];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 479);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[104];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 660);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[105];

    assert_eq!(script.id, 410);
    assert_eq!(script.local_variable_offset, 482);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[106];

    assert_eq!(script.id, 326);
    assert_eq!(script.local_variable_offset, 617);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[107];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 666);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[108];

    assert_eq!(script.id, 452);
    assert_eq!(script.local_variable_offset, 491);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[109];

    assert_eq!(script.id, 350);
    assert_eq!(script.local_variable_offset, 497);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[110];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 499);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[111];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 502);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[112];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 505);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[113];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 508);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[114];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 510);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[115];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 512);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[116];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 514);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[117];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 516);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[118];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 672);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[119];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 622);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[120];

    assert_eq!(script.id, 350);
    assert_eq!(script.local_variable_offset, 518);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[121];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 623);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[122];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 624);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[123];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 625);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[124];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 520);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[125];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 522);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[126];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 524);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[127];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 526);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[128];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 528);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[129];

    assert_eq!(script.id, 450);
    assert_eq!(script.local_variable_offset, 530);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[130];

    assert_eq!(script.id, 423);
    assert_eq!(script.local_variable_offset, 537);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[131];

    assert_eq!(script.id, 451);
    assert_eq!(script.local_variable_offset, 548);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[132];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 557);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[133];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[134];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[135];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[136];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[137];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[138];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[139];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[140];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[141];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[142];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[143];

    assert_eq!(script.id, 433);
    assert_eq!(script.local_variable_offset, 626);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[144];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 638);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[145];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 559);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[146];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 678);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[147];

    assert_eq!(script.id, 455);
    assert_eq!(script.local_variable_offset, 639);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[148];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 684);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[149];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 690);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[150];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 646);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[151];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 647);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[152];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 648);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[153];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 696);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[154];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 702);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[155];

    assert_eq!(script.id, 410);
    assert_eq!(script.local_variable_offset, 561);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[156];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 708);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[157];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 714);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[158];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 720);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[159];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 726);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[160];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 732);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[161];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 570);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[162];

    assert_eq!(script.id, 328);
    assert_eq!(script.local_variable_offset, 649);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[163];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 572);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[164];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 574);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[165];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 576);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[166];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 578);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[167];

    assert_eq!(script.id, 429);
    assert_eq!(script.local_variable_offset, 828);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[168];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 738);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[169];

    assert_eq!(script.id, 692);
    assert_eq!(script.local_variable_offset, 744);
    assert_eq!(script.local_variable_count, 17);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[170];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 580);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[171];

    assert_eq!(script.id, 326);
    assert_eq!(script.local_variable_offset, 650);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[172];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 582);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[173];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 584);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[174];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 586);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[175];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 761);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[176];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 767);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[177];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 773);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[178];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 779);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[179];

    assert_eq!(script.id, 410);
    assert_eq!(script.local_variable_offset, 588);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[180];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 597);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[181];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 600);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[182];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 785);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[183];

    assert_eq!(script.id, 410);
    assert_eq!(script.local_variable_offset, 603);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[184];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 791);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[185];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 797);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[186];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 803);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[187];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 809);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[188];

    assert_eq!(script.id, 326);
    assert_eq!(script.local_variable_offset, 655);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[189];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 612);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[190];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 815);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_newr2_map_save_scripts() {
    let decompressed = try_gunzip_buffer(NEWR2_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 6);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 12);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 18);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 1272);
    assert_eq!(script.local_variable_offset, 24);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 458);
    assert_eq!(script.local_variable_offset, 28);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 460);
    assert_eq!(script.local_variable_offset, 29);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 461);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 30);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 40);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 50);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 60);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 70);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 648);
    assert_eq!(script.local_variable_offset, 80);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 411);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 84);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 94);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 104);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 114);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 124);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 134);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 1048);
    assert_eq!(script.local_variable_offset, 144);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 1211);
    assert_eq!(script.local_variable_offset, 922);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 150);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 160);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 170);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 180);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 190);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 200);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 210);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 220);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 1042);
    assert_eq!(script.local_variable_offset, 230);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 647);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 473);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 649);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 1096);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 236);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 242);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 248);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 254);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 260);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 266);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 272);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 278);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 284);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 1209);
    assert_eq!(script.local_variable_offset, 923);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 1209);
    assert_eq!(script.local_variable_offset, 924);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 1209);
    assert_eq!(script.local_variable_offset, 925);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 1209);
    assert_eq!(script.local_variable_offset, 926);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 1210);
    assert_eq!(script.local_variable_offset, 927);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 1207);
    assert_eq!(script.local_variable_offset, 928);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 1207);
    assert_eq!(script.local_variable_offset, 929);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 1207);
    assert_eq!(script.local_variable_offset, 930);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 1207);
    assert_eq!(script.local_variable_offset, 931);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 1207);
    assert_eq!(script.local_variable_offset, 932);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 1207);
    assert_eq!(script.local_variable_offset, 933);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 1207);
    assert_eq!(script.local_variable_offset, 934);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 1207);
    assert_eq!(script.local_variable_offset, 935);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 1208);
    assert_eq!(script.local_variable_offset, 936);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 1208);
    assert_eq!(script.local_variable_offset, 937);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 1211);
    assert_eq!(script.local_variable_offset, 938);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 1211);
    assert_eq!(script.local_variable_offset, 939);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 1211);
    assert_eq!(script.local_variable_offset, 940);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 1211);
    assert_eq!(script.local_variable_offset, 941);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 1211);
    assert_eq!(script.local_variable_offset, 942);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 1211);
    assert_eq!(script.local_variable_offset, 943);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 1211);
    assert_eq!(script.local_variable_offset, 944);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 1204);
    assert_eq!(script.local_variable_offset, 945);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 1211);
    assert_eq!(script.local_variable_offset, 946);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 1048);
    assert_eq!(script.local_variable_offset, 290);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 1211);
    assert_eq!(script.local_variable_offset, 947);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 1211);
    assert_eq!(script.local_variable_offset, 948);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 1272);
    assert_eq!(script.local_variable_offset, 296);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 1272);
    assert_eq!(script.local_variable_offset, 300);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 304);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 321);
    assert_eq!(script.local_variable_offset, 902);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 319);
    assert_eq!(script.local_variable_offset, 912);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 325);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 319);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 325);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 445);
    assert_eq!(script.local_variable_offset, 806);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[85];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 310);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[86];

    assert_eq!(script.id, 325);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[87];

    assert_eq!(script.id, 325);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[88];

    assert_eq!(script.id, 325);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[89];

    assert_eq!(script.id, 346);
    assert_eq!(script.local_variable_offset, 808);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[90];

    assert_eq!(script.id, 443);
    assert_eq!(script.local_variable_offset, 913);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[91];

    assert_eq!(script.id, 645);
    assert_eq!(script.local_variable_offset, 313);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[92];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 317);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[93];

    assert_eq!(script.id, 319);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[94];

    assert_eq!(script.id, 326);
    assert_eq!(script.local_variable_offset, 813);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[95];

    assert_eq!(script.id, 346);
    assert_eq!(script.local_variable_offset, 818);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[96];

    assert_eq!(script.id, 326);
    assert_eq!(script.local_variable_offset, 823);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[97];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 319);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[98];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 828);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[99];

    assert_eq!(script.id, 452);
    assert_eq!(script.local_variable_offset, 331);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[100];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 337);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[101];

    assert_eq!(script.id, 350);
    assert_eq!(script.local_variable_offset, 349);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[102];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 829);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[103];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 351);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[104];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 363);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[105];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 375);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[106];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 387);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[107];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 830);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[108];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 399);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[109];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 411);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[110];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 831);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[111];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 832);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[112];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 413);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[113];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 833);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[114];

    assert_eq!(script.id, 328);
    assert_eq!(script.local_variable_offset, 834);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[115];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 425);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[116];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 437);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[117];

    assert_eq!(script.id, 325);
    assert_eq!(script.local_variable_offset, 921);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[118];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 449);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[119];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 452);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[120];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 455);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[121];

    assert_eq!(script.id, 410);
    assert_eq!(script.local_variable_offset, 458);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[122];

    assert_eq!(script.id, 410);
    assert_eq!(script.local_variable_offset, 467);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[123];

    assert_eq!(script.id, 410);
    assert_eq!(script.local_variable_offset, 476);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[124];

    assert_eq!(script.id, 452);
    assert_eq!(script.local_variable_offset, 485);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[125];

    assert_eq!(script.id, 410);
    assert_eq!(script.local_variable_offset, 491);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[126];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 500);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[127];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 835);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[128];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 502);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[129];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 505);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[130];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 517);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[131];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 519);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[132];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 522);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[133];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 525);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[134];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 528);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[135];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 531);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[136];

    assert_eq!(script.id, 410);
    assert_eq!(script.local_variable_offset, 534);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[137];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 543);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[138];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 555);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[139];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 567);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[140];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 579);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[141];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 591);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[142];

    assert_eq!(script.id, 319);
    assert_eq!(script.local_variable_offset, 911);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[143];

    assert_eq!(script.id, 340);
    assert_eq!(script.local_variable_offset, 836);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[144];

    assert_eq!(script.id, 320);
    assert_eq!(script.local_variable_offset, 837);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[145];

    assert_eq!(script.id, 325);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[146];

    assert_eq!(script.id, 1118);
    assert_eq!(script.local_variable_offset, 594);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[147];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 598);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[148];

    assert_eq!(script.id, 1119);
    assert_eq!(script.local_variable_offset, 600);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[149];

    assert_eq!(script.id, 430);
    assert_eq!(script.local_variable_offset, 838);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[150];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 608);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[151];

    assert_eq!(script.id, 1085);
    assert_eq!(script.local_variable_offset, 610);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[152];

    assert_eq!(script.id, 424);
    assert_eq!(script.local_variable_offset, 844);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[153];

    assert_eq!(script.id, 445);
    assert_eq!(script.local_variable_offset, 851);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[154];

    assert_eq!(script.id, 340);
    assert_eq!(script.local_variable_offset, 853);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[155];

    assert_eq!(script.id, 339);
    assert_eq!(script.local_variable_offset, 854);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[156];

    assert_eq!(script.id, 340);
    assert_eq!(script.local_variable_offset, 855);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[157];

    assert_eq!(script.id, 339);
    assert_eq!(script.local_variable_offset, 856);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[158];

    assert_eq!(script.id, 339);
    assert_eq!(script.local_variable_offset, 857);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[159];

    assert_eq!(script.id, 418);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[160];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 858);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[161];

    assert_eq!(script.id, 1119);
    assert_eq!(script.local_variable_offset, 613);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[162];

    assert_eq!(script.id, 328);
    assert_eq!(script.local_variable_offset, 859);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[163];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 621);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[164];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 623);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[165];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 625);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[166];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 627);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[167];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 629);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[168];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 631);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[169];

    assert_eq!(script.id, 350);
    assert_eq!(script.local_variable_offset, 633);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[170];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 635);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[171];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 637);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[172];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 639);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[173];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 641);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[174];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 643);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[175];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 645);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[176];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 647);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[177];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 649);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[178];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 651);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[179];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 653);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[180];

    assert_eq!(script.id, 328);
    assert_eq!(script.local_variable_offset, 860);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[181];

    assert_eq!(script.id, 346);
    assert_eq!(script.local_variable_offset, 861);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[182];

    assert_eq!(script.id, 325);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[183];

    assert_eq!(script.id, 346);
    assert_eq!(script.local_variable_offset, 866);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[184];

    assert_eq!(script.id, 346);
    assert_eq!(script.local_variable_offset, 871);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[185];

    assert_eq!(script.id, 434);
    assert_eq!(script.local_variable_offset, 655);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[186];

    assert_eq!(script.id, 328);
    assert_eq!(script.local_variable_offset, 876);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[187];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 877);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[188];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 878);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[189];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 663);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[190];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 665);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[191];

    assert_eq!(script.id, 417);
    assert_eq!(script.local_variable_offset, 667);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[192];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 879);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[193];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 669);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[194];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 671);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[195];

    assert_eq!(script.id, 418);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[196];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 673);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[197];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 685);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[198];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 697);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[199];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 709);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[200];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 721);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[201];

    assert_eq!(script.id, 1049);
    assert_eq!(script.local_variable_offset, 733);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[202];

    assert_eq!(script.id, 1086);
    assert_eq!(script.local_variable_offset, 738);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[203];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 740);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[204];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 742);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[205];

    assert_eq!(script.id, 346);
    assert_eq!(script.local_variable_offset, 880);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[206];

    assert_eq!(script.id, 346);
    assert_eq!(script.local_variable_offset, 885);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[207];

    assert_eq!(script.id, 442);
    assert_eq!(script.local_variable_offset, 903);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[208];

    assert_eq!(script.id, 419);
    assert_eq!(script.local_variable_offset, 890);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[209];

    assert_eq!(script.id, 1119);
    assert_eq!(script.local_variable_offset, 754);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[210];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 762);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[211];

    assert_eq!(script.id, 347);
    assert_eq!(script.local_variable_offset, 774);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[212];

    assert_eq!(script.id, 432);
    assert_eq!(script.local_variable_offset, 786);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[213];

    assert_eq!(script.id, 1049);
    assert_eq!(script.local_variable_offset, 798);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[214];

    assert_eq!(script.id, 327);
    assert_eq!(script.local_variable_offset, 803);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_newr3_map_save_scripts() {
    let decompressed = try_gunzip_buffer(NEWR3_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 713);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 712);
    assert_eq!(script.local_variable_offset, 1);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 1041);
    assert_eq!(script.local_variable_offset, 2);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 1212);
    assert_eq!(script.local_variable_offset, 214);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 657);
    assert_eq!(script.local_variable_offset, 8);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 461);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 1212);
    assert_eq!(script.local_variable_offset, 215);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 1041);
    assert_eq!(script.local_variable_offset, 9);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 1212);
    assert_eq!(script.local_variable_offset, 216);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 1212);
    assert_eq!(script.local_variable_offset, 217);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 15);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 25);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 35);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 45);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 55);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 65);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 75);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 85);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 95);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 105);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 115);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 125);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 1078);
    assert_eq!(script.local_variable_offset, 135);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 1212);
    assert_eq!(script.local_variable_offset, 218);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 1212);
    assert_eq!(script.local_variable_offset, 219);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 1212);
    assert_eq!(script.local_variable_offset, 220);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 416);
    assert_eq!(script.local_variable_offset, 193);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 425);
    assert_eq!(script.local_variable_offset, 202);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 426);
    assert_eq!(script.local_variable_offset, 138);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 144);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 188);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 427);
    assert_eq!(script.local_variable_offset, 146);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 440);
    assert_eq!(script.local_variable_offset, 194);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 150);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 152);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 189);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 154);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 156);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 158);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 160);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 162);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 164);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 166);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 168);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 190);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 170);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 191);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 172);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 174);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 176);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 402);
    assert_eq!(script.local_variable_offset, 192);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 336);
    assert_eq!(script.local_variable_offset, 178);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 1147);
    assert_eq!(script.local_variable_offset, 180);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 1147);
    assert_eq!(script.local_variable_offset, 182);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 1147);
    assert_eq!(script.local_variable_offset, 184);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 329);
    assert_eq!(script.local_variable_offset, 186);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_newrst_map_save_scripts() {
    let decompressed = try_gunzip_buffer(NEWRST_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 30);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 40);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 50);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 60);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 70);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 80);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 90);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 100);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 110);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 1043);
    assert_eq!(script.local_variable_offset, 120);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 1217);
    assert_eq!(script.local_variable_offset, 312);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 1219);
    assert_eq!(script.local_variable_offset, 313);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 1219);
    assert_eq!(script.local_variable_offset, 314);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 1219);
    assert_eq!(script.local_variable_offset, 315);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 1219);
    assert_eq!(script.local_variable_offset, 316);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 1218);
    assert_eq!(script.local_variable_offset, 317);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 1218);
    assert_eq!(script.local_variable_offset, 318);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 847);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 222);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 228);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 441);
    assert_eq!(script.local_variable_offset, 304);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 234);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 436);
    assert_eq!(script.local_variable_offset, 126);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 323);
    assert_eq!(script.local_variable_offset, 240);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 241);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 247);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 299);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 412);
    assert_eq!(script.local_variable_offset, 311);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 323);
    assert_eq!(script.local_variable_offset, 253);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 134);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 138);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 142);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 146);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 150);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 154);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 158);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 254);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 162);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 166);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 170);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 174);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 178);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 182);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 186);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 190);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 194);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 198);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 202);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 206);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 210);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 214);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 337);
    assert_eq!(script.local_variable_offset, 218);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 323);
    assert_eq!(script.local_variable_offset, 260);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 261);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 435);
    assert_eq!(script.local_variable_offset, 292);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 267);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 273);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 279);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 435);
    assert_eq!(script.local_variable_offset, 293);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 348);
    assert_eq!(script.local_variable_offset, 285);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 323);
    assert_eq!(script.local_variable_offset, 291);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_raiders_map_1_map_save_scripts() {
    let decompressed = try_gunzip_buffer(RAIDERS1_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);
}

#[test]
fn parses_raiders_map_2_map_save_scripts() {
    let decompressed = try_gunzip_buffer(RAIDERS2_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 1100);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 1100);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 1100);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 98);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 104);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 110);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 312);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 264);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 156);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 144);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 126);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 132);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 138);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 294);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 270);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 180);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 192);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 162);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 168);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 186);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 150);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 174);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 222);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 228);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 198);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 204);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 210);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 300);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 216);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 234);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 240);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 246);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 276);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 282);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 288);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 252);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 258);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, 306);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[85];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[86];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[87];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[88];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[89];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[90];

    assert_eq!(script.id, 1100);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[91];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[92];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[93];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[94];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[95];

    assert_eq!(script.id, 1156);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[96];

    assert_eq!(script.id, 1156);
    assert_eq!(script.local_variable_offset, 7);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[97];

    assert_eq!(script.id, 797);
    assert_eq!(script.local_variable_offset, 318);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[98];

    assert_eq!(script.id, 1295);
    assert_eq!(script.local_variable_offset, 325);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[99];

    assert_eq!(script.id, 1295);
    assert_eq!(script.local_variable_offset, 326);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[100];

    assert_eq!(script.id, 1295);
    assert_eq!(script.local_variable_offset, 327);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[101];

    assert_eq!(script.id, 1295);
    assert_eq!(script.local_variable_offset, 328);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[102];

    assert_eq!(script.id, 1295);
    assert_eq!(script.local_variable_offset, 329);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[103];

    assert_eq!(script.id, 1295);
    assert_eq!(script.local_variable_offset, 330);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[104];

    assert_eq!(script.id, 1294);
    assert_eq!(script.local_variable_offset, 331);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[105];

    assert_eq!(script.id, 1294);
    assert_eq!(script.local_variable_offset, 332);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[106];

    assert_eq!(script.id, 1294);
    assert_eq!(script.local_variable_offset, 333);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[107];

    assert_eq!(script.id, 1294);
    assert_eq!(script.local_variable_offset, 334);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[108];

    assert_eq!(script.id, 1294);
    assert_eq!(script.local_variable_offset, 335);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[109];

    assert_eq!(script.id, 1294);
    assert_eq!(script.local_variable_offset, 336);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[110];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[111];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[112];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[113];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[114];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[115];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[116];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[117];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[118];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[119];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[120];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[121];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[122];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[123];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[124];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[125];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[126];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[127];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[128];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[129];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[130];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[131];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[132];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[133];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[134];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[135];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[136];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[137];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[138];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[139];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[140];

    assert_eq!(script.id, 1099);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[141];

    assert_eq!(script.id, 794);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[142];

    assert_eq!(script.id, 794);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[143];

    assert_eq!(script.id, 796);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[144];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 14);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[145];

    assert_eq!(script.id, 794);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[146];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[147];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[148];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[149];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[150];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[151];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[152];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[153];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[154];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[155];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[156];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[157];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[158];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[159];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[160];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[161];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[162];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[163];

    assert_eq!(script.id, 1142);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[164];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 26);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[165];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 32);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[166];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 38);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[167];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 44);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[168];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 50);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[169];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 56);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[170];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 62);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[171];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 68);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[172];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 74);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[173];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 80);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[174];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 86);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[175];

    assert_eq!(script.id, 795);
    assert_eq!(script.local_variable_offset, 92);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[176];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 121);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_denbus2_map_1_save_scripts() {
    let decompressed = try_gunzip_buffer(REDDOWN_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 942);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 17);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 27);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 37);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 47);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 57);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 67);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 77);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 938);
    assert_eq!(script.local_variable_offset, 87);
    assert_eq!(script.local_variable_count, 2);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 942);
    assert_eq!(script.local_variable_offset, 89);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 942);
    assert_eq!(script.local_variable_offset, 96);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 103);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 109);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 119);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 129);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 139);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 149);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 159);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 169);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 179);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 189);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 199);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 209);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 219);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 942);
    assert_eq!(script.local_variable_offset, 229);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 236);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 242);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 248);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 254);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 260);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 266);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 272);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 278);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 284);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 290);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 296);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 302);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 308);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 314);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 320);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 376);
    assert_eq!(script.local_variable_offset, 326);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 958);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 1050);
    assert_eq!(script.local_variable_offset, 332);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 985);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 1272);
    assert_eq!(script.local_variable_offset, 340);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 688);
    assert_eq!(script.local_variable_offset, 686);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 822);
    assert_eq!(script.local_variable_offset, 696);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 522);
    assert_eq!(script.local_variable_offset, 709);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 1113);
    assert_eq!(script.local_variable_offset, 719);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 807);
    assert_eq!(script.local_variable_offset, 344);
    assert_eq!(script.local_variable_count, 17);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 689);
    assert_eq!(script.local_variable_offset, 729);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 809);
    assert_eq!(script.local_variable_offset, 742);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 681);
    assert_eq!(script.local_variable_offset, 377);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 694);
    assert_eq!(script.local_variable_offset, 390);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 409);
    assert_eq!(script.local_variable_offset, 361);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 409);
    assert_eq!(script.local_variable_offset, 367);
    assert_eq!(script.local_variable_count, 6);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 400);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 410);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 420);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 937);
    assert_eq!(script.local_variable_offset, 430);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 438);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 448);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 458);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 468);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 478);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 488);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 498);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 508);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 823);
    assert_eq!(script.local_variable_offset, 518);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 531);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 541);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 551);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 561);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 571);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 1148);
    assert_eq!(script.local_variable_offset, 581);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 591);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 601);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 893);
    assert_eq!(script.local_variable_offset, 373);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 808);
    assert_eq!(script.local_variable_offset, 611);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[85];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[86];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[87];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[88];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[89];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[90];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 626);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[91];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 636);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[92];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 646);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[93];

    assert_eq!(script.id, 1113);
    assert_eq!(script.local_variable_offset, 656);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[94];

    assert_eq!(script.id, 1113);
    assert_eq!(script.local_variable_offset, 666);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[95];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 676);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_redding_mine_entrance_map_save_scripts() {
    let decompressed = try_gunzip_buffer(REDMENT_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 5);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 15);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 25);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 30);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 35);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 40);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 45);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 50);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 55);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 60);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 65);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 70);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 75);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 80);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 85);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 958);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 90);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 95);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 100);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 110);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 120);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 130);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 140);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 150);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 160);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 170);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 180);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 190);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 200);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 210);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 220);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 230);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 240);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 250);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 260);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 270);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 280);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 290);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 300);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 310);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 320);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 330);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 340);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 958);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 958);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 1023);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 68);
    assert_eq!(script.local_variable_offset, 350);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 687);
    assert_eq!(script.local_variable_offset, 634);
    assert_eq!(script.local_variable_count, 17);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 691);
    assert_eq!(script.local_variable_offset, 651);
    assert_eq!(script.local_variable_count, 17);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 692);
    assert_eq!(script.local_variable_offset, 668);
    assert_eq!(script.local_variable_count, 17);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 729);
    assert_eq!(script.local_variable_offset, 355);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 685);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 821);
    assert_eq!(script.local_variable_offset, 695);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 695);
    assert_eq!(script.local_variable_offset, 708);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 533);
    assert_eq!(script.local_variable_offset, 364);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 718);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 728);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 738);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 748);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 758);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 768);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 533);
    assert_eq!(script.local_variable_offset, 379);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 778);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 788);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 454);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[74];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 464);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[75];

    assert_eq!(script.id, 677);
    assert_eq!(script.local_variable_offset, 474);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[76];

    assert_eq!(script.id, 1143);
    assert_eq!(script.local_variable_offset, 486);
    assert_eq!(script.local_variable_count, 25);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[77];

    assert_eq!(script.id, 533);
    assert_eq!(script.local_variable_offset, 394);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[78];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 511);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[79];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 521);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[80];

    assert_eq!(script.id, 693);
    assert_eq!(script.local_variable_offset, 531);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[81];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 544);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[82];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 554);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[83];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 564);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[84];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 574);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[85];

    assert_eq!(script.id, 533);
    assert_eq!(script.local_variable_offset, 409);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[86];

    assert_eq!(script.id, 806);
    assert_eq!(script.local_variable_offset, 584);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[87];

    assert_eq!(script.id, 533);
    assert_eq!(script.local_variable_offset, 424);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[88];

    assert_eq!(script.id, 533);
    assert_eq!(script.local_variable_offset, 439);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[89];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 594);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[90];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 604);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[91];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 614);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[92];

    assert_eq!(script.id, 690);
    assert_eq!(script.local_variable_offset, 624);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[93];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 803);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_arroyo_caves_1_save_scripts() {
    let decompressed = try_gunzip_buffer(REDMTUN_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 939);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 18);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 939);
    assert_eq!(script.local_variable_offset, 3);
    assert_eq!(script.local_variable_count, 3);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_arroyo_caves_2_save_scripts() {
    let decompressed = try_gunzip_buffer(REDWAME_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 167);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 1275);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 669);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 28);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 38);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 48);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 58);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 68);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 78);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 88);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 98);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 108);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 118);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 128);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 138);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 148);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 669);
    assert_eq!(script.local_variable_offset, 158);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 1272);
    assert_eq!(script.local_variable_offset, 166);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 175);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_v15ent_map_save_scripts() {
    let decompressed = try_gunzip_buffer(V15ENT_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 566);
    assert_eq!(script.local_variable_offset, 2);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 10);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 826);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 30);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 511);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 552);
    assert_eq!(script.local_variable_offset, 93);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 564);
    assert_eq!(script.local_variable_offset, 81);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 555);
    assert_eq!(script.local_variable_offset, 40);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 563);
    assert_eq!(script.local_variable_offset, 48);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 562);
    assert_eq!(script.local_variable_offset, 56);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 567);
    assert_eq!(script.local_variable_offset, 64);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 565);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 565);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 565);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 565);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 565);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 565);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 559);
    assert_eq!(script.local_variable_offset, 72);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_vault15_secret_entrance_map_map_save_scripts() {
    let decompressed = try_gunzip_buffer(V15SENT_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 167);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 576);
    assert_eq!(script.local_variable_offset, 1);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 610);
    assert_eq!(script.local_variable_offset, 9);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 610);
    assert_eq!(script.local_variable_offset, 17);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 25);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 825);
    assert_eq!(script.local_variable_offset, 35);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 0);
    assert_eq!(script.local_variable_offset, 55);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_arroyo_bridge_2_save_scripts() {
    let decompressed = try_gunzip_buffer(VCTYCOCL_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 1254);
    assert_eq!(script.local_variable_offset, 387);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 1254);
    assert_eq!(script.local_variable_offset, 388);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 1254);
    assert_eq!(script.local_variable_offset, 389);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 1239);
    assert_eq!(script.local_variable_offset, 390);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 1239);
    assert_eq!(script.local_variable_offset, 391);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 1240);
    assert_eq!(script.local_variable_offset, 392);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 1252);
    assert_eq!(script.local_variable_offset, 381);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 1253);
    assert_eq!(script.local_variable_offset, 393);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 1253);
    assert_eq!(script.local_variable_offset, 394);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 389);
    assert_eq!(script.local_variable_offset, 275);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 283);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 842);
    assert_eq!(script.local_variable_offset, 293);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 389);
    assert_eq!(script.local_variable_offset, 301);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 309);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 842);
    assert_eq!(script.local_variable_offset, 319);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 116);
    assert_eq!(script.local_variable_offset, 327);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 337);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 347);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 389);
    assert_eq!(script.local_variable_offset, 357);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 389);
    assert_eq!(script.local_variable_offset, 365);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 842);
    assert_eq!(script.local_variable_offset, 373);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 127);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 20);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 20);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 28);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 128);
    assert_eq!(script.local_variable_offset, 38);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 1047);
    assert_eq!(script.local_variable_offset, 51);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 1047);
    assert_eq!(script.local_variable_offset, 59);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 1047);
    assert_eq!(script.local_variable_offset, 67);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 1047);
    assert_eq!(script.local_variable_offset, 75);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 842);
    assert_eq!(script.local_variable_offset, 83);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 955);
    assert_eq!(script.local_variable_offset, 91);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 100);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 108);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 1047);
    assert_eq!(script.local_variable_offset, 116);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 124);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 1047);
    assert_eq!(script.local_variable_offset, 134);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 142);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 150);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 158);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 166);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 842);
    assert_eq!(script.local_variable_offset, 176);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 184);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 192);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 200);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 208);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 842);
    assert_eq!(script.local_variable_offset, 216);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 389);
    assert_eq!(script.local_variable_offset, 224);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 232);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 842);
    assert_eq!(script.local_variable_offset, 242);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 250);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 258);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 974);
    assert_eq!(script.local_variable_offset, 268);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_vctyctyd_map_save_scripts() {
    let decompressed = try_gunzip_buffer(VCTYCTYD_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 669);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 1272);
    assert_eq!(script.local_variable_offset, 8);
    assert_eq!(script.local_variable_count, 4);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 901);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 957);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 980);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 1197);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 1198);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 1236);
    assert_eq!(script.local_variable_offset, 268);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 1237);
    assert_eq!(script.local_variable_offset, 269);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 1241);
    assert_eq!(script.local_variable_offset, 270);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 1242);
    assert_eq!(script.local_variable_offset, 271);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 1242);
    assert_eq!(script.local_variable_offset, 272);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 1242);
    assert_eq!(script.local_variable_offset, 273);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 1238);
    assert_eq!(script.local_variable_offset, 274);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 1255);
    assert_eq!(script.local_variable_offset, 275);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 1255);
    assert_eq!(script.local_variable_offset, 276);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 900);
    assert_eq!(script.local_variable_offset, 12);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 111);
    assert_eq!(script.local_variable_offset, 212);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 91);
    assert_eq!(script.local_variable_offset, 220);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 110);
    assert_eq!(script.local_variable_offset, 228);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 88);
    assert_eq!(script.local_variable_offset, 13);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 89);
    assert_eq!(script.local_variable_offset, 21);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 88);
    assert_eq!(script.local_variable_offset, 31);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 88);
    assert_eq!(script.local_variable_offset, 39);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 88);
    assert_eq!(script.local_variable_offset, 47);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 114);
    assert_eq!(script.local_variable_offset, 239);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 841);
    assert_eq!(script.local_variable_offset, 248);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 202);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 92);
    assert_eq!(script.local_variable_offset, 256);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 387);
    assert_eq!(script.local_variable_offset, 55);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 387);
    assert_eq!(script.local_variable_offset, 63);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 387);
    assert_eq!(script.local_variable_offset, 71);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 387);
    assert_eq!(script.local_variable_offset, 79);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 88);
    assert_eq!(script.local_variable_offset, 87);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 841);
    assert_eq!(script.local_variable_offset, 136);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 112);
    assert_eq!(script.local_variable_offset, 144);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 89);
    assert_eq!(script.local_variable_offset, 95);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 89);
    assert_eq!(script.local_variable_offset, 105);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 841);
    assert_eq!(script.local_variable_offset, 152);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 612);
    assert_eq!(script.local_variable_offset, 202);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 952);
    assert_eq!(script.local_variable_offset, 115);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 687);
    assert_eq!(script.local_variable_offset, 160);
    assert_eq!(script.local_variable_count, 17);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 109);
    assert_eq!(script.local_variable_offset, 192);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 571);
    assert_eq!(script.local_variable_offset, 177);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 89);
    assert_eq!(script.local_variable_offset, 126);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_arroyo_bridge_1_save_scripts() {
    let decompressed = try_gunzip_buffer(VCTYDWTN_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 1251);
    assert_eq!(script.local_variable_offset, 419);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 1254);
    assert_eq!(script.local_variable_offset, 420);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 663);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 902);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 956);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 1141);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 1244);
    assert_eq!(script.local_variable_offset, 421);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 1244);
    assert_eq!(script.local_variable_offset, 422);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 1243);
    assert_eq!(script.local_variable_offset, 423);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 1245);
    assert_eq!(script.local_variable_offset, 424);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 1245);
    assert_eq!(script.local_variable_offset, 425);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 1245);
    assert_eq!(script.local_variable_offset, 426);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 1246);
    assert_eq!(script.local_variable_offset, 427);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 1247);
    assert_eq!(script.local_variable_offset, 428);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 1248);
    assert_eq!(script.local_variable_offset, 429);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 1248);
    assert_eq!(script.local_variable_offset, 430);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 1248);
    assert_eq!(script.local_variable_offset, 431);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 1248);
    assert_eq!(script.local_variable_offset, 432);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 1248);
    assert_eq!(script.local_variable_offset, 433);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 1248);
    assert_eq!(script.local_variable_offset, 434);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 1250);
    assert_eq!(script.local_variable_offset, 435);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 1249);
    assert_eq!(script.local_variable_offset, 436);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 1251);
    assert_eq!(script.local_variable_offset, 437);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 1251);
    assert_eq!(script.local_variable_offset, 438);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 1254);
    assert_eq!(script.local_variable_offset, 439);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[25];

    assert_eq!(script.id, 1254);
    assert_eq!(script.local_variable_offset, 440);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[26];

    assert_eq!(script.id, 899);
    assert_eq!(script.local_variable_offset, 1);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[27];

    assert_eq!(script.id, 888);
    assert_eq!(script.local_variable_offset, 2);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[28];

    assert_eq!(script.id, 120);
    assert_eq!(script.local_variable_offset, 199);
    assert_eq!(script.local_variable_count, 14);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[29];

    assert_eq!(script.id, 115);
    assert_eq!(script.local_variable_offset, 213);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[30];

    assert_eq!(script.id, 93);
    assert_eq!(script.local_variable_offset, 3);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[31];

    assert_eq!(script.id, 93);
    assert_eq!(script.local_variable_offset, 15);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[32];

    assert_eq!(script.id, 123);
    assert_eq!(script.local_variable_offset, 27);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[33];

    assert_eq!(script.id, 964);
    assert_eq!(script.local_variable_offset, 222);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[34];

    assert_eq!(script.id, 118);
    assert_eq!(script.local_variable_offset, 446);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[35];

    assert_eq!(script.id, 389);
    assert_eq!(script.local_variable_offset, 229);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[36];

    assert_eq!(script.id, 389);
    assert_eq!(script.local_variable_offset, 237);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[37];

    assert_eq!(script.id, 389);
    assert_eq!(script.local_variable_offset, 245);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[38];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 253);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[39];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 261);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[40];

    assert_eq!(script.id, 123);
    assert_eq!(script.local_variable_offset, 36);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[41];

    assert_eq!(script.id, 668);
    assert_eq!(script.local_variable_offset, 45);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[42];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 269);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[43];

    assert_eq!(script.id, 122);
    assert_eq!(script.local_variable_offset, 279);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[44];

    assert_eq!(script.id, 1127);
    assert_eq!(script.local_variable_offset, 292);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[45];

    assert_eq!(script.id, 388);
    assert_eq!(script.local_variable_offset, 302);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[46];

    assert_eq!(script.id, 388);
    assert_eq!(script.local_variable_offset, 310);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[47];

    assert_eq!(script.id, 388);
    assert_eq!(script.local_variable_offset, 318);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[48];

    assert_eq!(script.id, 388);
    assert_eq!(script.local_variable_offset, 326);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[49];

    assert_eq!(script.id, 1047);
    assert_eq!(script.local_variable_offset, 334);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[50];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 342);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[51];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 350);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[52];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 358);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[53];

    assert_eq!(script.id, 1047);
    assert_eq!(script.local_variable_offset, 366);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[54];

    assert_eq!(script.id, 126);
    assert_eq!(script.local_variable_offset, 374);
    assert_eq!(script.local_variable_count, 13);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[55];

    assert_eq!(script.id, 1047);
    assert_eq!(script.local_variable_offset, 387);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[56];

    assert_eq!(script.id, 1046);
    assert_eq!(script.local_variable_offset, 395);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[57];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 403);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[58];

    assert_eq!(script.id, 1046);
    assert_eq!(script.local_variable_offset, 411);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[59];

    assert_eq!(script.id, 1046);
    assert_eq!(script.local_variable_offset, 67);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[60];

    assert_eq!(script.id, 1046);
    assert_eq!(script.local_variable_offset, 75);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[61];

    assert_eq!(script.id, 839);
    assert_eq!(script.local_variable_offset, 83);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[62];

    assert_eq!(script.id, 840);
    assert_eq!(script.local_variable_offset, 94);
    assert_eq!(script.local_variable_count, 11);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[63];

    assert_eq!(script.id, 1047);
    assert_eq!(script.local_variable_offset, 105);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[64];

    assert_eq!(script.id, 971);
    assert_eq!(script.local_variable_offset, 113);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[65];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 125);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[66];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 133);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[67];

    assert_eq!(script.id, 843);
    assert_eq!(script.local_variable_offset, 141);
    assert_eq!(script.local_variable_count, 8);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[68];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 149);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[69];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 159);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[70];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 169);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[71];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 179);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[72];

    assert_eq!(script.id, 89);
    assert_eq!(script.local_variable_offset, 57);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[73];

    assert_eq!(script.id, 90);
    assert_eq!(script.local_variable_offset, 189);
    assert_eq!(script.local_variable_count, 10);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}

#[test]
fn parses_vault_city_vault_map_save_scripts() {
    let decompressed = try_gunzip_buffer(VCTYVLT_SAVE.to_vec());
    let (_, _, scripts) = map_save(&decompressed);

    let script = &scripts[0];

    assert_eq!(script.id, 167);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[1];

    assert_eq!(script.id, 167);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[2];

    assert_eq!(script.id, 167);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[3];

    assert_eq!(script.id, 1144);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(1).unwrap());

    let script = &scripts[4];

    assert_eq!(script.id, 837);
    assert_eq!(script.local_variable_offset, 80);
    assert_eq!(script.local_variable_count, 5);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[5];

    assert_eq!(script.id, 838);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[6];

    assert_eq!(script.id, 836);
    assert_eq!(script.local_variable_offset, 86);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[7];

    assert_eq!(script.id, 836);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[8];

    assert_eq!(script.id, 836);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[9];

    assert_eq!(script.id, 836);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[10];

    assert_eq!(script.id, 836);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[11];

    assert_eq!(script.id, 844);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[12];

    assert_eq!(script.id, 972);
    assert_eq!(script.local_variable_offset, -1);
    assert_eq!(script.local_variable_count, 0);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[13];

    assert_eq!(script.id, 981);
    assert_eq!(script.local_variable_offset, 0);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[14];

    assert_eq!(script.id, 981);
    assert_eq!(script.local_variable_offset, 1);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[15];

    assert_eq!(script.id, 1088);
    assert_eq!(script.local_variable_offset, 2);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[16];

    assert_eq!(script.id, 1088);
    assert_eq!(script.local_variable_offset, 9);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[17];

    assert_eq!(script.id, 1088);
    assert_eq!(script.local_variable_offset, 16);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[18];

    assert_eq!(script.id, 1088);
    assert_eq!(script.local_variable_offset, 23);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[19];

    assert_eq!(script.id, 1088);
    assert_eq!(script.local_variable_offset, 30);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[20];

    assert_eq!(script.id, 1152);
    assert_eq!(script.local_variable_offset, 37);
    assert_eq!(script.local_variable_count, 7);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[21];

    assert_eq!(script.id, 1166);
    assert_eq!(script.local_variable_offset, 85);
    assert_eq!(script.local_variable_count, 1);
    assert_eq!(script.script_type, ScriptTagType::try_from(3).unwrap());

    let script = &scripts[22];

    assert_eq!(script.id, 94);
    assert_eq!(script.local_variable_offset, 44);
    assert_eq!(script.local_variable_count, 15);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[23];

    assert_eq!(script.id, 117);
    assert_eq!(script.local_variable_offset, 68);
    assert_eq!(script.local_variable_count, 12);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());

    let script = &scripts[24];

    assert_eq!(script.id, 673);
    assert_eq!(script.local_variable_offset, 59);
    assert_eq!(script.local_variable_count, 9);
    assert_eq!(script.script_type, ScriptTagType::try_from(4).unwrap());
}
