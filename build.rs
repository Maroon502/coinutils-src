use std::env;
use std::path::{Path, PathBuf};

use coin_build_tools::{coinbuilder, link, utils};

const LIB_NAME: &str = "CoinUtils";

fn main() {
    println!(
        "cargo:rerun-if-changed={}_lib_sources.txt",
        LIB_NAME.to_ascii_lowercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_STATIC",
        LIB_NAME.to_ascii_uppercase()
    );
    println!(
        "cargo:rerun-if-env-changed=CARGO_{}_SYSTEM",
        LIB_NAME.to_ascii_uppercase()
    );

    let want_system = utils::want_system(LIB_NAME);

    if want_system && link::link_lib_system_if_supported(LIB_NAME) {
        let coinflags = vec!["COINUTILS".to_string()];
        coinbuilder::print_metadata(Vec::new(), coinflags);
        return;
    }

    if !Path::new(&format!("{}/LICENSE", LIB_NAME)).exists() {
        utils::update_submodules(env::var("CARGO_MANIFEST_DIR").unwrap());
    }
    build_lib_and_link();
}

fn build_lib_and_link() {
    let src_dir = format!(
        "{}",
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(LIB_NAME)
            .join(LIB_NAME)
            .join("src")
            .display()
    );

    let includes_dir = vec![src_dir.clone()];

    let lib_sources = include_str!("coinutils_lib_sources.txt")
        .trim()
        .split('\n')
        .map(|file| format!("{}/{}", src_dir, file.trim()))
        .collect::<Vec<String>>();

    let coinflags = vec!["COINUTILS".to_string()];

    coinbuilder::print_metadata(includes_dir.clone(), coinflags.clone());

    let mut config = coinbuilder::init_builder();
    coinflags.iter().for_each(|flag| {
        config.define(&format!("COIN_HAS_{}", flag), None);
    });
    config.includes(includes_dir);
    config.files(lib_sources);

    config.compile(LIB_NAME);
}
