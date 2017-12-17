pub mod desktop_entry;
mod parse;

use std::path::{Path, PathBuf};
use std::io::Read;
use std::fs::File;
use self::desktop_entry::{DesktopEntry, EntryType};
use self::parse::{parse_entry_type, parse_if_starts_with};

pub fn get_entries_in_dirs(paths: &'static [&'static str]) -> Vec<DesktopEntry> {
    return parse_dirs(paths);
}

pub fn get_entries_in_dirs_filtered_by(paths: &'static [&'static str], entry_type: EntryType) -> Vec<DesktopEntry> {
    return parse_dirs(paths)
        .into_iter()
        .filter(|entry: &DesktopEntry|
            entry.entry_type == entry_type)
        .collect();
}

fn parse_dirs(paths: &'static [&'static str]) -> Vec<DesktopEntry> {
    let mut desktop_entries: Vec<DesktopEntry> = Vec::new();

    for path in paths {
        let dirs = Path::new(path).read_dir().expect("read_dir call failed");

        for entry in dirs {
            let entry_path = entry.expect("entry in dirs failed").path();
            if entry_path.is_file() {
                match parse_file(&entry_path) {
                    Ok(entry) => desktop_entries.push(entry),
                    Err(error) => println!("{:?}", error)
                }
            }
        }
    }

    desktop_entries
}

pub fn parse_file(file_path: &PathBuf) -> Result<DesktopEntry, &'static str> {
    let mut contents = String::new();

    File::open(file_path)
        .expect("File::open failed")
        .read_to_string(&mut contents)
        .expect("File::read_to_string failed");

    let mut entry_type: Option<EntryType> = None;
    let mut name: Option<String> = None;
    let mut exec: Option<String> = None;
    let mut comment: Option<String> = None;

    for line in contents.lines() {
        if parse_if_starts_with(line, &mut name, "Name=") {
            continue;
        }


        if parse_if_starts_with(line, &mut exec, "Exec=") {
            continue;
        }


        if parse_if_starts_with(line, &mut comment, "Comment=") {
            continue;
        }

        let mut type_str: Option<String> = None;
        parse_if_starts_with(line, &mut type_str, "Type=");

        match type_str {
            Some(val) => {
                entry_type = match parse_entry_type(&val) {
                    Ok(val) => Some(val),
                    Err(_) => None
                };
            }
            _ => ()
        }
    }

    if name.is_none() {
        return Err("Name key is required");
    }

    if entry_type.is_none() {
        return Err("Type key is required");
    }

    Ok(DesktopEntry {
        name: name.unwrap_or("NO NAME".to_string()),
        entry_type: entry_type.unwrap_or(EntryType::Directory),
        exec,
        comment,
    })
}

