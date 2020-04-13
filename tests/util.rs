use std::fs;
use std::path::PathBuf;
use rust_succ::util::PathSucker;

fn current_dir() -> PathBuf {
    std::env::current_dir().expect("Cannot get current dir").to_owned()
}

#[test]
fn get_set_default() {
    let mut sucker = PathSucker::default();
    let mut dir = current_dir();

    dir.pop();

    sucker.set_default(&dir);

    assert_eq!(sucker.get_default(), dir);
}

#[test]
fn get_set_current() {
    let mut sucker = PathSucker::default();
    let path = PathBuf::from("bla");
    let dir = current_dir();

    sucker.set_current(&path);

    assert_eq!(sucker.get_current(), dir.join(path));
}

#[test]
#[should_panic]
fn set_relative_path() {
    let mut sucker = PathSucker::default();
    let dir = PathBuf::from("tests");

    sucker.set_default(&dir);
}

#[test]
fn absolutize_path() {
    let mut sucker = PathSucker::default();
    let relative = PathBuf::from("tests/util.rs");

    let dir = current_dir();
    let new_path = sucker.absolute_path(&relative);

    assert_eq!(dir.join("tests/util.rs"), new_path);
}

#[test]
fn exists() {
    let mut succ_path = PathSucker::default();

    fs::File::create("file.succ").expect("Error creating the succ file");

    succ_path.set_current(&PathBuf::from("faile.suuc"));
    assert!(!succ_path.exists());

    succ_path.set_current(&PathBuf::from("faile"));
    assert!(!succ_path.exists());

    succ_path.set_current(&PathBuf::from("file.suuc"));
    assert!(succ_path.exists());

    succ_path.set_current(&PathBuf::from("file"));
    assert!(succ_path.exists());

    fs::remove_file("file.succ").expect("Error removing the succ file");
}
