// BOJ 4779 [Cantoring Along]
// Supported by GitHub Copilot

use std::io::{self, Read, Write};
fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}
fn next<T>(it: &mut std::iter::Peekable<std::str::SplitAsciiWhitespace>) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace().peekable();

    let mut s = "-".to_string();
    for i in 0..12 {
        s = s.clone() + (0..3usize.pow(i as u32)).map(|_| " ").collect::<String>().as_str() + s.as_str();
    }

    while it.peek().is_some() {
        writeln!(so, "{}", s.chars().take(3usize.pow(next(&mut it))).collect::<String>()).ok();
    }
}
