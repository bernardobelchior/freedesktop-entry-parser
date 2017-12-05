extern crate desktop_entry_parser;


#[cfg(test)]
mod tests {
    use desktop_entry_parser::*;

    #[test]
    fn it_parses_dirs() {
        let paths : &'static[&'static str] = &["/usr/share/applications/"];

        let desktop_entries = parse_dirs(paths);

        for desktop_entry in desktop_entries {
            print!("{:?}", desktop_entry);
        }
    }
}
