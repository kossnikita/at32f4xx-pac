use std::env;
use std::fs;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
        println!("cargo:rustc-link-search={}", out.display());
        let device_file = if env::var_os("CARGO_FEATURE_AT32A403A").is_some() {
            "src/at32a403a/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F402").is_some() {
            "src/at32f402/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F403").is_some() {
            "src/at32f403/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F403A").is_some() {
            "src/at32f403a/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F405").is_some() {
            "src/at32f405/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F407").is_some() {
            "src/at32f407/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F413").is_some() {
            "src/at32f413/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F415").is_some() {
            "src/at32f415/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F421").is_some() {
            "src/at32f421/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F423").is_some() {
            "src/at32f423/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F425").is_some() {
            "src/at32f425/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F435").is_some() {
            "src/at32f435/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32F437").is_some() {
            "src/at32f437/device.x"
        } else if env::var_os("CARGO_FEATURE_AT32WB415").is_some() {
            "src/at32wb415/device.x"
        } else {
            panic!("No device features selected");
        };
        fs::copy(device_file, out.join("device.x")).unwrap();
        println!("cargo:rerun-if-changed={}", device_file);
    }
    println!("cargo:rerun-if-changed=build.rs");
}
