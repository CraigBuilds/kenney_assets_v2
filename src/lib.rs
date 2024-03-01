use std::path::PathBuf;

use home;

pub fn assets_path() -> PathBuf {
    let mut cargo_path = home::cargo_home().expect("Could not find cargo home");
    cargo_path.push("git");
    cargo_path.push("checkouts");
    cargo_path.push("assets");
    //find all the directories in the assets folder that start with "kenney_assets_v2"
    let all_dirs = cargo_path.read_dir().expect(&format!("Could not read {:?}", cargo_path));
    let mut matching_dirs = all_dirs.filter(|dir| {
        let dir = dir.as_ref().expect("Could not read dir");
        let name = dir.file_name();
        let name = name.to_str().expect("Could not convert dir name to str");
        name.starts_with("kenney_assets_v2")
    });
    //use the first matching directory
    let matching_dir = matching_dirs.next().expect("Could not find matching directory");
    let matching_dir = matching_dir.expect("Could not read matching directory");
    matching_dir.path()
}