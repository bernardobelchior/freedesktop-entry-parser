extern crate desktop_entry_parser;

#[cfg(test)]
mod tests {
    use desktop_entry_parser::*;
    use desktop_entry_parser::desktop_entry::EntryType;
    use std::path::Path;

    #[test]
    fn it_parses_dirs() {
        let paths: &'static [&'static str] = &["/usr/share/applications/"];

        let desktop_entries = get_entries_in_dirs(paths);
        assert!(desktop_entries.len() > 0, true);
    }

    #[test]
    fn it_gets_all_entries() {
        let paths: &'static [&'static str] = &["/usr/share/applications/"];

        let desktop_entries = get_entries_in_dirs(paths);

        for desktop_entry in desktop_entries {
            println!("{}", desktop_entry.name);
        }
    }

    #[test]
    fn it_gets_applications() {
        let paths: &'static [&'static str] = &["/usr/share/applications/"];

        let application_entries = get_entries_in_dirs_filtered_by(paths, EntryType::Application);

        for desktop_entry in application_entries {
            //print!("{:?}", desktop_entry);
        }
    }

    #[test]
    fn it_parses_application() {
        let path = Path::new("./tests/entries/test_app.desktop");
        let file = parse_file(&path.to_path_buf()).unwrap();

        assert_eq!(file.entry_type, EntryType::Application);
        assert_eq!(file.name, "Test App");
        assert_eq!(file.exec.unwrap(), "test-app");
        assert_eq!(file.comment.unwrap(), "A test application comment");
    }

    #[test]
    fn it_parses_link() {
        let path = Path::new("./tests/entries/test_link.desktop");
        let file = parse_file(&path.to_path_buf()).unwrap();

        assert_eq!(file.entry_type, EntryType::Link);
        assert_eq!(file.name, "Test Link");
        //assert_eq!(file.url.unwrap(), "test-app");
        assert_eq!(file.comment.unwrap(), "A test link comment");
    }
}
