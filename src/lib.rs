use std::path::PathBuf;

pub fn assets_path() -> PathBuf {
    let mut cargo_path = home::cargo_home().expect("Could not find cargo home");
    cargo_path.push("git");
    cargo_path.push("checkouts");
    //find all the directories in the assets folder that start with "kenney_assets_v2"
    let all_git_checkout_dirs = cargo_path.read_dir().expect(&format!("Could not read {:?}", cargo_path));
    let kenney_git_checkout_dirs = all_git_checkout_dirs.filter(|dir| {
        let dir = dir.as_ref().expect("Could not read dir");
        let name = dir.file_name();
        let name = name.to_str().expect("Could not convert dir name to str");
        name.starts_with("kenney_assets_v2")
    });
    //this contains all dirs that start with "kenney_assets_v2". use the one with the latest date modified
    let latest_dir = kenney_git_checkout_dirs.max_by_key(|dir| {
        let dir = dir.as_ref().expect("Could not read dir");
        let metadata = dir.metadata().expect("Could not read metadata");
        metadata.modified().expect("Could not read modified")
    }).expect("Could not find any kenney_assets_v2 dirs");
    //this contains sub folders with more commit hashes, again use the latest one
    let latest_dir = latest_dir.expect("Could not read latest dir");
    let latest_dir = latest_dir.path();
    let all_commit_dirs = latest_dir.read_dir().expect(&format!("Could not read {:?}", latest_dir));
    let latest_commit_dir = all_commit_dirs.max_by_key(|dir| {
        let dir = dir.as_ref().expect("Could not read dir");
        let metadata = dir.metadata().expect("Could not read metadata");
        metadata.modified().expect("Could not read modified")
    }).expect("Could not find any commit dirs");
    //this now contains the actual git project. return the assets folder
    let latest_commit_dir = latest_commit_dir.expect("Could not read latest commit dir");
    let latest_commit_dir = latest_commit_dir.path();
    let assets_path = latest_commit_dir.join("assets");
    assets_path
}

pub fn assets_path_str() -> String {
    assets_path().to_str().expect("Could not convert assets path to str").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assets_path() {
        let path = assets_path();
        assert!(path.exists());
    }
}