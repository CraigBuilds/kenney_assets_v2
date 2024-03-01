use std::path::PathBuf;

use home;

pub fn assets_path() -> PathBuf {
    let mut cargo_path = home::cargo_home().expect("Could not find cargo home");
    cargo_path.push("git");
    cargo_path.push("assets");
    cargo_path
}