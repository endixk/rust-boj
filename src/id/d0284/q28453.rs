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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    (0..n).for_each(|_| {
        match next::<u16>(&mut it) {
            x if x > 299 => write!(so, "1 ").unwrap(),
            x if x > 274 => write!(so, "2 ").unwrap(),
            x if x > 249 => write!(so, "3 ").unwrap(),
            _ => write!(so, "4 ").unwrap(),
        }
    });
}