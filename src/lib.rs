use std::{
    fs::File,
    io::{self, BufRead},
};

/// # Panics
///
/// Will panic if the input file does not exist
pub fn by_line<F>(file: &str, delim: Option<u8>, mut f: F)
where
    F: FnMut(Vec<u8>),
{
    let file = File::open(file).unwrap_or_else(|e| panic!("Can not open input file {file}: {e}"));
    let reader = io::BufReader::new(file);

    if let Some(c) = delim {
        for line in reader.split(c).map_while(Result::ok) {
            f(line);
        }
    } else {
        for line in reader.lines().map_while(Result::ok) {
            f(line.into());
        }
    }
}
