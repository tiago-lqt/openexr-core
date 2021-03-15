extern crate bindgen;

use std::path::PathBuf;
use std::{env, path::Path};

fn main() {
    let openexr_dir =
        env::var("OPENEXR_DIR").expect("'OPENEXR_DIR' envvar is not defined");

    let openexr_lib_path = Path::new(&openexr_dir).join("lib");
    let openexr_include_path =
        Path::new(&openexr_dir).join("include").join("OpenEXR");

    //  panic!(
    //     "\nopenexr_dir: {}\nopenexr_lib_path: {}\nopenexr_include_path: {}",
    //     openexr_dir,
    //     openexr_lib_path.display(),
    //     openexr_include_path.display(),
    // );

    // Tell cargo to tell rustc to link the OpenEXRCore .lib
    println!("cargo:rustc-link-search={}", openexr_lib_path.display());

    // println!("cargo:rustc-link-lib=static={}", "Iex-2_5_static");
    // println!("cargo:rustc-link-lib=static={}", "IexMath-2_5_static");
    // println!("cargo:rustc-link-lib=static={}", "IlmThread-2_5_static");
    // println!("cargo:rustc-link-lib=static={}", "OpenEXR-2_5_static");
    println!("cargo:rustc-link-lib={}", "OpenEXRCore-2_5");
    // println!("cargo:rustc-link-lib=static={}", "OpenEXRUtil-2_5_static");
    // println!("cargo:rustc-link-lib={}", "OpenEXRUtil-2_5");
    // println!("cargo:rustc-link-lib={}", "IexMath-2_5");
    // println!("cargo:rustc-link-lib={}", "Iex-2_5");
    // println!("cargo:rustc-link-lib={}", "Imath-3_0");
    // println!("cargo:rustc-link-lib={}", "IlmThread-2_5");
    // println!("cargo:rustc-link-lib={}", "Half-2_5");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-F{}", openexr_include_path.display()))
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // generate newtypes for enums
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
        })
        // we'll do these manually
        .blacklist_type("exr_attr_chromaticities_t")
        .blacklist_type("exr_attr_keycode_t")
        .blacklist_type("exr_attr_rational_t")
        .blacklist_type("exr_attr_timecode_t")
        .blacklist_type("exr_attr_tiledesc_t")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
