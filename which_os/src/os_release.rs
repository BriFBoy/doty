use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader, ErrorKind},
};

pub(super) fn pars_os_release() -> Result<HashMap<String, String>, io::Error> {
    let file = File::open("/etc/os-release")?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(v) => {
                let (key, value) = pars_line(&v).ok_or(io::Error::from(ErrorKind::InvalidInput))?;
                map.insert(key, value);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(map)
}

fn pars_line(line: &str) -> Option<(String, String)> {
    let (key, mut value) = line.split_once("=")?;

    if value.starts_with("\"") {
        let mut chars = value.chars();
        chars.next();
        value = chars.as_str();
    }

    if value.ends_with("\"") {
        let mut chars = value.chars();
        chars.next_back();
        value = chars.as_str();
    }

    Some((String::from(key), String::from(value)))
}
