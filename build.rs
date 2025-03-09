use std::{
    fs::create_dir_all,
    path::{Path, PathBuf},
    process::Command,
};

const LOCAL_INCLUDE_DIR: &str = "./vendor/local/include";
const GF_COMPLETE_HEADER_WRAPPER: &str = "./vendor/gf_complete_wrapper.h";
const JERASURE_HEADER_WRAPPER: &str = "./vendor/jerasure_wrapper.h";
const GF_MODULE_DIR: &str = "./vendor/gf-complete";
const JERASURE_MODULE_DIR: &str = "./vendor/jerasure";

fn main() {
    // Set rerun-if-changed
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=vendor");

    // Set libraries to link
    // WARNING: The order of the following lines is important
    if cfg!(feature = "link_static") {
        println!("-- Linking static libraries");
        println!("cargo:rustc-link-lib=static=Jerasure");
        println!("cargo:rustc-link-lib=static=gf_complete");
    } else {
        println!("cargo:rustc-link-lib=Jerasure");
        println!("cargo:rustc-link-lib=gf_complete");
    }

    // Build libraries
    if cfg!(feature = "bundle") {
        if !cfg!(feature = "link_static") {
            println!(
                "cargo::warning=It is discouraged to link shared libraries when bundling. See more: https://doc.rust-lang.org/cargo/reference/build-scripts.html#rustc-link-search"
            );
        }
        build_from_source();
    } else {
        // try to link the system libraries
        bindgen_sys();
    }
}

fn bindgen_sys() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let binding_out_dir = out_dir.join("bindings");
    let gf_wrapper_file_path = PathBuf::from(GF_COMPLETE_HEADER_WRAPPER);
    let gf_out_file_path = binding_out_dir.join("gf_complete.rs");
    let jr_wrapper_file_path = PathBuf::from(JERASURE_HEADER_WRAPPER);
    let jr_out_file_path = binding_out_dir.join("jerasure.rs");

    println!(
        "-- Creating bindings directory {}",
        binding_out_dir.display()
    );
    std::fs::create_dir_all(&binding_out_dir).expect("fail to create bindings directory");

    // bindgen for gf-complete
    println!(
        "-- Generate bindings: {} => {}",
        gf_wrapper_file_path.display(),
        gf_out_file_path.display()
    );

    bindgen::Builder::default()
        .header(gf_wrapper_file_path.to_str().unwrap())
        .allowlist_item("gf_.*")
        .allowlist_item("GF_.*")
        .impl_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_comments(true)
        .generate()
        .expect("Unable to generate jerasure bindings")
        .write_to_file(gf_out_file_path)
        .expect("Couldn't write bindings!");

    // bindgen for jerasure
    println!(
        "-- Generate bindings: {} => {}",
        jr_wrapper_file_path.display(),
        jr_out_file_path.display()
    );
    bindgen::Builder::default()
        .header(jr_wrapper_file_path.to_str().unwrap())
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
        .write_to_file(jr_out_file_path)
        .expect("Couldn't write bindings!");
}

fn build_from_source() {
    build_gf_complete();
    build_jerasure();
    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(std::env::var_os("OUT_DIR").unwrap())
            .join("lib")
            .canonicalize()
            .unwrap()
            .display()
    );
    // Make bindings
    bindgen_gf_complete();
    bindgen_jerasure();
}

fn build_gf_complete() {
    const _MIN_VERSION: &str = "2.0";
    const LIB_NAME: &str = "gf-complete";

    // Submodule directory containing upstream source files (readonly)
    let module_dir = std::fs::canonicalize(GF_MODULE_DIR).expect("gf-complete directory not found");

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

    // Run `autoreconf`
    println!(
        "sh: [autoreconf --force --install -I m4] in {}",
        src_dir.display()
    );
    let output = Command::new("autoreconf")
        .current_dir(src_dir.clone())
        .args(["--force", "--install", "-I", "m4"])
        .output()
        .unwrap();
    println!("autoreconf: {}", String::from_utf8_lossy(&output.stdout));
    eprintln!("autoreconf: {}", String::from_utf8_lossy(&output.stderr));

    // Build using auto-tools
    println!(
        "sh: [./configure --prefix={}] in {}",
        build_dir.display(),
        src_dir.display()
    );
    let _install_root_dir = autotools::Config::new(src_dir)
        .enable_shared()
        .enable_static()
        .build();

    // cleanup the build dir
    println!("-- Removing build directory {}", build_dir.display());
    std::fs::remove_dir_all(build_dir).unwrap();
}

fn build_jerasure() {
    const _MIN_VERSION: &str = "2.0";
    const LIB_NAME: &str = "jerasure";

    // Submodule directory containing upstream source files (readonly)
    let module_dir =
        std::fs::canonicalize(JERASURE_MODULE_DIR).expect("jerasure src directory not found");

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

    // Run `autoreconf`
    println!(
        "sh: [autoreconf --force --install -I m4] in {}",
        src_dir.display()
    );
    let output = Command::new("autoreconf")
        .current_dir(src_dir.clone())
        .args(["--force", "--install", "-I", "m4"])
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
    let _install_root_dir = autotools::Config::new(src_dir.clone())
        .enable_shared()
        .enable_static()
        .cflag(flag)
        .build();

    // cleanup the build dir
    println!("-- Removing build directory {}", build_dir.display());
    std::fs::remove_dir_all(build_dir).unwrap();
}

fn bindgen_jerasure() {
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    // let include_dir = out_dir.join("include");
    // let include_dir = PathBuf::from(SYS_INCLUDE_DIR);
    let binding_out_dir = out_dir.join("bindings");
    let local_header_path = PathBuf::from(LOCAL_INCLUDE_DIR).canonicalize().unwrap();
    let header_wrapper = PathBuf::from(JERASURE_HEADER_WRAPPER);
    let out_file = binding_out_dir.join("jerasure.rs");

    println!(
        "-- Creating bindings directory {}",
        binding_out_dir.display()
    );
    std::fs::create_dir_all(&binding_out_dir).expect("fail to create bindings directory");

    println!(
        "-- Generate bindings: {} => {}",
        header_wrapper.display(),
        out_file.display()
    );
    bindgen::Builder::default()
        .clang_args(["-isystem", local_header_path.to_str().unwrap()])
        .header(header_wrapper.to_str().unwrap())
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
    let binding_out_dir = out_dir.join("bindings");
    let local_header_path = PathBuf::from(LOCAL_INCLUDE_DIR).canonicalize().unwrap();
    let out_file_path = binding_out_dir.join("gf_complete.rs");
    let header_wrapper_path = PathBuf::from(GF_COMPLETE_HEADER_WRAPPER);

    println!(
        "-- Creating bindings directory {}",
        binding_out_dir.display()
    );
    std::fs::create_dir_all(&binding_out_dir).expect("fail to create bindings directory");

    println!(
        "-- Generate bindings: [{}] => {}",
        header_wrapper_path.display(),
        out_file_path.display()
    );

    bindgen::Builder::default()
        .clang_args(["-isystem", local_header_path.to_str().unwrap()])
        .header(header_wrapper_path.to_str().unwrap())
        .allowlist_item("gf_.*")
        .allowlist_item("GF_.*")
        .impl_debug(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate_comments(true)
        .generate()
        .expect("Unable to generate jerasure bindings")
        .write_to_file(out_file_path)
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
