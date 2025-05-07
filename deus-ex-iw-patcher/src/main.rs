use std::{
    fs::{self, File},
    io::{self, Read},
    path::Path,
};

use clap::{Parser, Subcommand};

/// Patch Deus Ex: Invisible War resolution and FOV
#[derive(Parser)]
#[command(author, version, about, long_about = None, )]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set correct FOV in Engine binary
    PatchResolution {
        /// Path to Engine.d2u
        #[arg(value_name = "FILE", verbatim_doc_comment)]
        file: String,

        /// Horizontal resolution
        #[arg(short)]
        x: u32,

        /// Vertical resolution
        #[arg(short)]
        y: u32,
    },
}

fn approx_equal(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

// All the credits go to this post: https://www.wsgf.org/dr/deus-ex-invisible-war
fn fov_for_resolution(x: &u32, y: &u32) -> (f32, u32) {
    let aspect_ration = *x as f32 / *y as f32;
    let four_by_three = 4f32 / 3f32;
    let sixteen_by_ten = 16f32 / 10f32;
    let sixteen_by_nine = 16f32 / 9f32;
    let fifteen_by_nine = 15f32 / 9f32;

    // This seems way too complex :D
    if approx_equal(aspect_ration, four_by_three, 1e-10) {
        (85_f32, 67_u32)
    } else if approx_equal(aspect_ration, sixteen_by_ten, 1e-10) {
        (108_f32, 58_u32)
    } else if approx_equal(aspect_ration, sixteen_by_nine, 1e-10) {
        (120_f32, 53_u32)
    } else if approx_equal(aspect_ration, fifteen_by_nine, 1e-10) {
        (112.5_f32, 50_u32)
    } else {
        // dunno, just return default
        (85_f32, 67_u32)
    }
}

fn replace_all_bytes(buf: &mut Vec<u8>, from: &[u8], to: &[u8]) -> usize {
    let mut count = 0;
    let mut i = 0;

    while i <= buf.len().saturating_sub(from.len()) {
        if &buf[i..i + from.len()] == from {
            buf.splice(i..i + from.len(), to.iter().cloned());
            i += to.len(); // move past the inserted bytes
            count += 1;
        } else {
            i += 1;
        }
    }

    count
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::PatchResolution { file, x, y } => {
            let path = Path::new(file);
            let d2u_system_dir = path.parent().expect("File you gave has no parent, what?");
            let new_engine_path = d2u_system_dir.join("Engine.d2u.new");
            let recommendations = fov_for_resolution(x, y);

            // This is the hardcoded FOV in the Engine binary
            let pattern: &[u8] = &[0x00, 0x00, 0xAA, 0x42];
            let le_bytes: [u8; 4] = recommendations.0.to_le_bytes();

            let mut file = File::open(path)?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            let occurances = replace_all_bytes(&mut buffer, pattern, &le_bytes);

            // Engine should have had the pattern defined in three places, abort
            if occurances != 3 {
                panic!("Unknown Engine.d2u, are you using the correct file?");
            }

            // Copy original engine to retain file permissions and such
            fs::copy(path, &new_engine_path)
                .expect("Could not copy old engine as temp file, check permissions?");

            fs::write(new_engine_path, buffer)
                .expect("Could not write new engine file, check permissions?");

            println!("File patched, please set the following manually:\n");
            println!("In 'steamapps/common/Deus Ex Invisible War/System/Default.ini':");
            println!("  FOV__d={}", recommendations.1);
            println!("  AssumedUIScreenWidth__d={}\n", y);
            println!(
                "In 'pfx/drive_c/users/steamuser/Documents/Deus Ex - Invisible War/user.ini':"
            );
            println!("  FullscreenViewportY={}", y);
            println!("  FullscreenViewportX={}", x);

            Ok(())
        }
    }
}
