extern crate desktop_entry_parser;


#[cfg(test)]
mod tests {
    use desktop_entry_parser::*;

    #[test]
    fn it_parses_dirs() {
        let paths : &'static[&'static str] = &["/usr/share/applications/"];

        parse_dirs(paths);
    }
}
