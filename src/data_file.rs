use std::fs;
use std::io;
use std::path::PathBuf;

use crate::util::PathSucker;


struct DataFile {
    path: PathBuf,
    raw_data: String
}

impl DataFile {
    pub fn new(mut path: PathBuf, file: Option<&'static str>) -> Result<Self, Box<dyn Error>> {
        let mut sucker = PathSucker::default();

        sucker.set_current(&path);

        if !sucker.exists() {
            let mut current = sucker.get_current();

            if file.is_none() {
                current.pop();
                fs::create_dir_all(current)?;
                fs::File::create(sucker.get_current())?;
            } else {
                fs::create_dir_all(current)?;
                current.push(file.unwrap());
                fs::File::create(current)?;
                sucker.set_current(&current);
            }
        }

        Ok(Self {
            path: sucker.get_current(),
            raw_data: String::new()
        })
    }

    pub fn get_data(&self) -> String {
        if self.path.exists() {
            return fs::read_to_string(self.path).expect("Cannot read data from file");
        }

        String::new()
    }

    pub fn reload_data(&mut self) {
        let data = self.get_data();

        raw_data = data;
        // TODO parser
    }

    pub fn save_data(&self) {
        fs::self.path
    }

    pub fn get<T>(&self) -> T {

    }

    pub fn set(&mut self) {

    }
}
