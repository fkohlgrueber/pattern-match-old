extern crate compiletest_rs as compiletest;

use std::path::PathBuf;

fn run_mode(mode: &'static str) {
    let mut config = compiletest::Config::default();

    config.mode = mode.parse().expect("Invalid mode");
    config.src_base = PathBuf::from(format!("tests/{}", mode));
    config.target_rustcflags = Some(r"-L C:\Users\felix\.rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib\rustlib\x86_64-pc-windows-msvc\lib -L ../target/debug/ -L ../target/debug/deps/".to_owned());
    //config.link_deps(); // Populate config.target_rustcflags with dependencies on the path
    config.clean_rmeta(); // If your tests import the parent crate, this helps with E0464
    config.rustc_path = PathBuf::from("target/debug/pattern_match");

    std::env::set_var("LINTER_TESTMODE", "1");
    compiletest::run_tests(&config);
}

#[test]
fn compile_test() {
    run_mode("ui");
}
