use std::{
    fs::{self, File, OpenOptions},
    io::{BufWriter, Seek, SeekFrom, Write},
};

use clap::{Parser, Subcommand};
use flate2::{write::GzEncoder, Compression};

use crate::parser::{map_save, try_gunzip_buffer, Script};

slint::slint! {
    export component HelloWorld {
        Text {
            text: "hello world";
            color: green;
        }
    }
}

pub fn run_ui() {
    HelloWorld::new().unwrap().run().unwrap();
}

#[derive(Subcommand)]
enum Commands {
    /// Sets all NCR cops to friendly, fuck you sulik!
    FixNCRCopAggro,
}

/// Program to manipulate Fallout 2 saves
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the save file to load
    #[arg(short, long)]
    save_file_path: String,
}

const NCR_GUARD_AGGRO_LVAR_INDEX: usize = 5;
const GLOBAL_VARIABLE_START: usize = 0x00EC;

// FIXME(tatu): Holy fuck this code is horrible :D
//              I just wanted to get the NCR aggro reset working as quickly as possible.
fn ncr_cop_aggro_fix(save_file_path: String) {
    let content = fs::read(save_file_path).expect("could not read save file");

    let decompressed = try_gunzip_buffer(content);
    let (_, map_variables, scripts) = map_save(&decompressed);

    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .truncate(true)
            .open("NCR1.BAK")
            .expect("couldn't create new save");

        file.write_all(&decompressed)
            .expect("couldn't write new save");

        let ncr_guard_scripts: Vec<&Script> =
            scripts.iter().filter(|script| script.id == 447).collect();

        for script in &ncr_guard_scripts {
            let variable_start_offset =
                usize::try_from(script.local_variable_offset).expect("script should have offset");

            let variable_count =
                usize::try_from(script.local_variable_count).expect("script should have variables");

            let script_variables =
                map_variables.local_variables_by_offset(variable_start_offset, variable_count);

            let aggro_state = script_variables[NCR_GUARD_AGGRO_LVAR_INDEX];

            if aggro_state == 2 {
                let local_variable_start_offset =
                    GLOBAL_VARIABLE_START + map_variables.global_variables.len() * 4;

                let write_offset: u64 = u64::try_from(local_variable_start_offset).unwrap()
                    + u64::try_from(variable_start_offset).unwrap() * 4
                    + 5 * 4;

                println!("agro found, fixing at offset {write_offset}");

                file.seek(SeekFrom::Start(write_offset)).unwrap();
                let buff = [0, 0, 0, 0];

                file.write_all(&buff)
                    .expect("should have been able to write");
            }
        }
    }

    let bytes = fs::read("NCR1.BAK").unwrap();

    let file = File::create("NCR1.SAV_NEW").unwrap();
    let writer = BufWriter::new(file);
    let mut encoder = GzEncoder::new(writer, Compression::default());
    encoder.write_all(&bytes).unwrap();
    encoder.finish().unwrap();
}

pub fn run_terminal_ui() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::FixNCRCopAggro => ncr_cop_aggro_fix(cli.save_file_path),
    }
}
