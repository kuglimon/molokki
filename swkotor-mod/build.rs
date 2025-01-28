use std::{env, fs::File, path::Path};

use gl_generator::{Api, Fallbacks, Profile, Registry, StaticGenerator};

fn build_opengl_bindings() {
    let dest: String = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("gl_bindings_2_1.rs")).unwrap();

    let registry = Registry::new(
        Api::Gl,
        (2, 1), // or (3,2) with Profile::Compatibility
        Profile::Compatibility,
        Fallbacks::All,
        [],
    );

    // Then generate bindings that include the old fixed-function pipeline:
    registry.write_bindings(StaticGenerator, &mut file).unwrap();
}

fn main() {
    build_opengl_bindings();
}
