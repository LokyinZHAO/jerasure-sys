use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    const LIB_GF_FILE: &str = "gf_complete";
    const LIB_JR_FILE: &str = "Jerasure";

    // Build libraries
    build_gf_complete();
    build_jerasure();
    // WARNING: The order of the following lines is important
    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(std::env::var_os("OUT_DIR").unwrap())
            .join("lib")
            .canonicalize()
            .unwrap()
            .display()
    );
    println!("cargo:rustc-link-lib=static={}", LIB_JR_FILE);
    println!("cargo:rustc-link-lib=static={}", LIB_GF_FILE);

    // Make bindings
    bindgen_gf_complete();
    bindgen_jerasure();
}

fn build_gf_complete() {
    const _MIN_VERSION: &str = "2.0";
    const LIB_NAME: &str = "gf-complete";
    const MODULE_DIR: &str = "./vendor/gf-complete";

    // Submodule directory containing upstream source files (readonly)
    let module_dir = std::fs::canonicalize(MODULE_DIR).expect("gf-complete directory not found");

    // Copy source files to writable directory in `OUT_DIR`
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let src_dir = out_dir.join("src").join(LIB_NAME);
    let build_dir = out_dir.join("build");
    create_dir_all(&src_dir).unwrap_or_else(|_| panic!("Failed to create {}", src_dir.display()));
    println!(
        "-- Copying gf-complete source files to {}",
        src_dir.display()
    );
    cp_r(module_dir, src_dir.clone());

    // Run `autorecofig`
    println!(
        "sh: [autoreconf --force --install -I m4] in {}",
        src_dir.display()
    );
    let output = Command::new("autoreconf")
        .current_dir(src_dir.clone())
        .args(&["--force", "--install", "-I", "m4"])
        .output()
        .unwrap();
    println!("autoreconf: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("autoreconf: {}", String::from_utf8_lossy(&output.stderr));

    // Build using autotools
    println!(
        "sh: [./configure --prefix={}] in {}",
        build_dir.display(),
        src_dir.display()
    );
    let _install_root_dir = autotools::Config::new(src_dir).build();

    // cleanup the build dir
    println!("-- Removing build directory {}", build_dir.display());
    std::fs::remove_dir_all(build_dir).unwrap();
}

fn build_jerasure() {
    const _MIN_VERSION: &str = "2.0";
    const LIB_NAME: &str = "jerasure";
    const MODULE_DIR: &str = "./vendor/jerasure";

    // Submodule directory containing upstream source files (readonly)
    let module_dir = std::fs::canonicalize(MODULE_DIR).expect("jerasure src directory not found");

    // Copy source files to writable directory in `OUT_DIR`
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let src_dir = out_dir.join("src").join(LIB_NAME);
    let include_dir = out_dir.join("include");
    let lib_dir = out_dir.join("lib");
    let build_dir = out_dir.join("build");

    // copy source
    create_dir_all(&src_dir).unwrap_or_else(|_| panic!("Failed to create {}", src_dir.display()));
    println!("-- Copying jerasure source files to {}", src_dir.display());
    cp_r(module_dir, src_dir.clone());

    // Run `autorecofig`
    println!(
        "sh: [autoreconf --force --install -I m4] in {}",
        src_dir.display()
    );
    let output = Command::new("autoreconf")
        .current_dir(src_dir.clone())
        .args(&["--force", "--install", "-I", "m4"])
        .output()
        .unwrap();
    println!("autoreconf: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("autoreconf: {}", String::from_utf8_lossy(&output.stderr));

    // Build using autotools
    let flag = format!(
        "-L{} -I{}",
        lib_dir.canonicalize().unwrap().display(),
        include_dir.canonicalize().unwrap().display()
    );
    println!(
        "sh: [./configure --prefix={}] [$CFLAG={}] in {}",
        build_dir.display(),
        flag,
        src_dir.display()
    );
    let _install_root_dir = autotools::Config::new(src_dir.clone()).cflag(flag).build();

    // cleanup the build dir
    println!("-- Removing build directory {}", build_dir.display());
    std::fs::remove_dir_all(build_dir).unwrap();
}

fn bindgen_jerasure() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let include_dir = out_dir.join("include");
    let binding_dir = out_dir.join("bindings");
    let sys_header_path = include_dir.canonicalize().unwrap();
    let header_files = [
        PathBuf::from(&include_dir).join("jerasure.h"),
        PathBuf::from(&include_dir).join("galois.h"),
        PathBuf::from(&include_dir).join("reed_sol.h"),
        PathBuf::from(&include_dir).join("cauchy.h"),
        // PathBuf::from(&include_dir).join("liberation.h"),
    ];
    let out_file = binding_dir.join("jerasure.rs");

    println!("-- Creating bindings directory {}", binding_dir.display());
    std::fs::create_dir_all(&binding_dir).expect("fail to create bindings directory");

    println!(
        "-- Generate bindings: [{}] => {}",
        header_files
            .iter()
            .map(|f| f.display().to_string())
            .collect::<Vec<String>>()
            .join(", "),
        out_file.display()
    );
    bindgen::Builder::default()
        .clang_args(["-isystem", sys_header_path.to_str().unwrap()])
        .headers(header_files.iter().map(|f| f.to_str().unwrap()))
        .impl_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_item("jerasure_.*")
        .allowlist_item("galois_.*")
        .allowlist_item("cauchy_.*")
        .allowlist_item("reed_sol_.*")
        // .allowlist_item("liberation_.*")
        .generate_comments(true)
        .generate()
        .expect("Unable to generate jerasure bindings")
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}

fn bindgen_gf_complete() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let include_dir = out_dir.join("include");
    let binding_dir = out_dir.join("bindings");
    let sys_header_path = include_dir.canonicalize().unwrap();
    let header_files = [
        PathBuf::from(&include_dir).join("gf_complete.h"),
        PathBuf::from(&include_dir).join("gf_general.h"),
    ];
    let out_file = binding_dir.join("gf_complete.rs");

    println!("-- Creating bindings directory {}", binding_dir.display());
    std::fs::create_dir_all(&binding_dir).expect("fail to create bindings directory");

    println!(
        "-- Generate bindings: [{}] => {}",
        header_files
            .iter()
            .map(|f| f.display().to_string())
            .collect::<Vec<String>>()
            .join(", "),
        out_file.display()
    );

    bindgen::Builder::default()
        .clang_args(["-isystem", sys_header_path.to_str().unwrap()])
        .headers(header_files.iter().map(|f| f.to_str().unwrap()))
        .allowlist_item("gf_.*")
        .allowlist_item("GF_.*")
        .impl_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_comments(true)
        .generate()
        .expect("Unable to generate jerasure bindings")
        .write_to_file(out_file)
        .expect("Couldn't write bindings!");
}

fn cp_r(from: impl AsRef<Path>, to: impl AsRef<Path>) {
    for e in from.as_ref().read_dir().unwrap() {
        let e = e.unwrap();
        let from = e.path();
        let to = to.as_ref().join(e.file_name());
        if e.file_type().unwrap().is_dir() {
            std::fs::create_dir_all(&to).unwrap();
            cp_r(&from, &to);
        } else {
            println!("cp: {} => {}", from.display(), to.display());
            std::fs::copy(&from, &to).unwrap();
        }
    }
}
