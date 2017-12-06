extern crate desktop_entry_parser;

#[cfg(test)]
mod tests {
    use desktop_entry_parser::*;
    use desktop_entry_parser::desktop_entry::EntryType;

    #[test]
    fn it_parses_dirs() {
        let paths: &'static [&'static str] = &["/usr/share/applications/"];

        let desktop_entries = get_entries_in_dirs(paths);
        assert!(desktop_entries.len() > 0, true);
    }

    #[test]
    fn it_gets_applications() {
        let paths: &'static [&'static str] = &["/usr/share/applications/"];

        let application_entries = get_entries_in_dirs_filtered_by(paths, EntryType::Application);

        for desktop_entry in application_entries {
            print!("{:?}", desktop_entry);
        }
    }
}
