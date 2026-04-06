// Module to hold the generated opengl functions.
//
// As we are using an old version of opengl with
// the game, we generate the bindings in build.rs
// import them here, binding the path to this module.
//
// In the future, this module may hold some compatibility
// fixes if needed.
//

include!(concat!(env!("OUT_DIR"), "/gl_bindings_2_1.rs"));
