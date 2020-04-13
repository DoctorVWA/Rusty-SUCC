use std::env;
use std::path::PathBuf;


pub static FILE_EXT: &str = "succ";


pub struct PathSucker {
    current_path: PathBuf,
    default_path: PathBuf
}

// Default path as the current dir
impl Default for PathSucker {
    fn default() -> Self {
        Self {
            current_path: PathBuf::from(""),
            default_path: env::current_dir().expect("Cannot get current dir")
        }
    }
}

// Default path class
impl PathSucker {
    // Get default path owned
    pub fn get_default(&self) -> PathBuf {
        self.default_path.to_owned()
    }

    // Get current path owned
    pub fn get_current(&self) -> PathBuf {
        self.current_path.to_owned()
    }

    // Set a new default path
    pub fn set_default(&mut self, path: &PathBuf) {
        if !path.is_absolute() {
            panic!("The default path should be absolute");
        }

        self.default_path = path.to_owned();
    }

    // Set current path. It can be relative, it will be absolutized
    pub fn set_current(&mut self, mut path: &PathBuf) {
        path.set_extension(FILE_EXT);
        self.current_path = self.absolute_path(path)
    }

    // Make a path absolute, or append a relative one to the default path
    pub fn absolute_path(&mut self, path: &PathBuf) -> PathBuf {
        if path.is_absolute() {
            return path.to_owned();
        }

        let mut absolute = self.get_default();

        absolute.push(path);
        absolute
    }
    
    // Check if the file on the path actually exists
    pub fn exists(&mut self) -> bool {
        self.current_path.exists()
    }
}
