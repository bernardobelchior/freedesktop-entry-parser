use desktop_entry::EntryType;

pub fn parse_entry_type(s: &str) -> Result<EntryType, &'static str> {
    match s {
        "Application" => Ok(EntryType::Application),
        "Link" => Ok(EntryType::Link),
        "Directory" => Ok(EntryType::Directory),
        _ => Err("Unknown entry type")
    }
}

pub fn parse_if_starts_with<'a>(line: &'a str, result: &mut Option<String>, starts_with: &'a str) {
    if line.starts_with(starts_with) {
        if let Some(val) = line.get(starts_with.len()..) {
            *result = Some(val.to_string());
        }
    };
}
