use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn by_line<F>(file: &str, mut f: F)
where
    F: FnMut(&str),
{
    let file = File::open(file).unwrap_or_else(|e| panic!("Can not open input file {file}: {e}"));
    let lines = io::BufReader::new(file).lines();

    // Consumes the iterator, returns an (Optional) String
    for line in lines.map_while(Result::ok) {
        f(&line);
    }
}
