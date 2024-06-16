use std::fs;

use clap::Parser;

use crate::parser::{dat2, try_decompress_dat2};

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

// FIXME(tatu): Make a command like fix and options for common fixes
/// Program to fix NCR guard agro in Fallout 2 saves
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the save file to load
    #[arg(short, long)]
    save_file_path: String,
}

pub fn run_terminal_ui() {
    let args = Args::parse();
    let content = fs::read(args.save_file_path).expect("could not read save file");

    let decompressed = try_decompress_dat2(content);
    let (map_save, map_variables, scripts) = dat2(&decompressed);

    dbg!(map_save);
    dbg!(map_variables);
    dbg!(scripts);
}
