extern crate cpp_build;
extern crate pkg_config;

fn main() {
    let lib: pkg_config::Library;

    // Checking if paludis library is installed
    {
        let l = pkg_config::find_library("paludis");

        if l.is_err() {
            panic!("Can't find paludis library !")
        } else {
            lib = l.unwrap();
        }
    }

    // Checking paludis version
    let version = lib.version.as_str();
    assert!(
        version == "3.0",
        "paludis library as the wrong version, the expected version is 3.0 instead of {version}"
    );

    // Getting paludis header path
    let include_path = lib.include_paths[0]
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    // let header_path = format!("{include_path}/paludis");

    // Building c++ wrappers
    cpp_build::Config::new()
        .include(&include_path)
        .build("src/bindings.rs");

    // Library linking
    println!("cargo:rustc-link-lib=paludis_3.0");
    println!("cargo:rustc-link-lib=paludisutil_3.0");
    println!("cargo:rustc-link-lib=paludisargs_3.0");
}
