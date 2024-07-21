fn main() {

    // build and install patomic
    let cmake_install_dir = cmake::build("patomic");
    let cmake_include_dir = cmake_install_dir.join("include");
    let cmake_lib_dir = cmake_install_dir.join("lib");

    // setup patomic cffi bindings
    let patomic_header_path = cmake_include_dir.join("patomic/include/patomic.h");
    let bindings = bindgen::Builder::default()
        .header(patomic_header_path.to_str().unwrap())
        .must_use_type(".*")
        .generate_cstr(true)
        .generate()
        .expect("Unable to generate bindings");

    // generate patomic cffi bindings
    let patomic_bindings_path = std::path::PathBuf::from("src/gen/cffi.rs");
    bindings
        .write_to_file(patomic_bindings_path)
        .expect("Couldn't write bindings!");
}
