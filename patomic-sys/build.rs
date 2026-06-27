fn main() {
    // build vendored patomic library statically
    let install_prefix = cmake::Config::new("vendor/patomic")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("CMAKE_INSTALL_LIBDIR", "lib")
        .build();

    // add CMake's LIB_DIR to search path
    let lib_dir = install_prefix.join("lib");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());

    // set "patomic" as expected library name
    println!("cargo:rustc-link-lib=static=patomic");

    // rebuild when needed
    println!("cargo:rerun-if-changed=vendor/patomic");
    println!("cargo:rerun-if-changed=build.rs");
}
