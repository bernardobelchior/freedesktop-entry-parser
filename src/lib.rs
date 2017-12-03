use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs::File;

pub fn parse_dirs(paths: &'static [&'static str]) {
    for path in paths {
        let dirs = Path::new(path).read_dir().expect("read_dir call failed");

        for entry in dirs {
            let entry = entry.unwrap();
            parse_file(entry.path());
        }
    }
}

pub fn parse_file(file_path: PathBuf) {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
}
