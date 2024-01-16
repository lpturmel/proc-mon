use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Find the macOS SDK path using xcrun
    let sdk_path = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("Failed to execute xcrun")
        .stdout;
    let sdk_path = String::from_utf8_lossy(&sdk_path).trim().to_string();

    // Construct the path to libproc.h within the SDK
    let libproc_path = format!("{}/usr/include/libproc.h", sdk_path);

    println!("cargo:rustc-link-lib=proc"); // link against libproc

    let bindings = bindgen::Builder::default()
        .header(libproc_path)
        .clang_arg(format!("-isysroot{}", sdk_path))
        .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
