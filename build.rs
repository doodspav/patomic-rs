#[cfg(feature = "bindgen")]
use std::fs;
#[cfg(feature = "bindgen")]
use std::path::{Path, PathBuf};

#[cfg(feature = "bindgen")]
fn generate_patomic_bindings(bindings_file: &Path, patomic_include_dir: &Path) {
    // ensure gen directory exists
    // bindgen will not create subdirectories for us
    fs::create_dir_all(bindings_file.parent().unwrap()).unwrap();

    // delete file if it already exists
    // bindgen will panic instead of overwriting existing files
    if bindings_file.exists() {
        fs::remove_file(bindings_file).unwrap();
    }

    // setup patomic cffi bindings
    let patomic_header_path = PathBuf::from(patomic_include_dir).join("patomic/patomic.h");
    let bindings = bindgen::Builder::default()
        .header(patomic_header_path.to_str().unwrap())
        .clang_arg(format!("-I{}", patomic_include_dir.display()))
        .must_use_type(".*")
        .generate_cstr(true)
        .allowlist_item("^(patomic|PATOMIC)_.*")
        .generate()
        .expect("Unable to generate patomic cffi bindings");

    // generate patomic cffi bindings
    bindings
        .write_to_file(bindings_file)
        .expect("Could not write patomic cffi bindings to file");
}

fn main() {
    // build and install patomic
    let cmake_install_dir = cmake::Config::new("patomic")
        .define("CMAKE_INSTALL_INCLUDEDIR", "include")
        .define("BUILD_SHARED_LIBS", "ON")
        .build();
    print!("{:?}", cmake_install_dir);

    // conditionally re-generate patomic cffi bindings
    #[cfg(feature = "bindgen")]
    {
        let bindings_file = Path::new("src/gen/cffi.rs");
        let cmake_include_dir = cmake_install_dir.join("include");
        generate_patomic_bindings(bindings_file, cmake_include_dir.as_path())
    }
}
