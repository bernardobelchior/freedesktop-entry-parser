use std::path::Path;

pub fn main() {
    let path = Path::new("/usr/share/applications/");

    let dirs = path.read_dir().expect("read_dir call failed");

    for entry in dirs {
        if let Ok(entry) = entry {
            println!("{:?}", entry.path());
        }
    }
}
