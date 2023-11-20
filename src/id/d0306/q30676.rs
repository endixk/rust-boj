use std::io::{self, Read, Write};
fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}
fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    writeln!(so, "{}", match next::<usize>(&mut it) {
        x if x >= 620 => "Red",
        x if x >= 590 => "Orange",
        x if x >= 570 => "Yellow",
        x if x >= 495 => "Green",
        x if x >= 450 => "Blue",
        x if x >= 425 => "Indigo",
        _ => "Violet",
    })?;

    Ok(())
}