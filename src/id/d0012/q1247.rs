// BOJ 1247 [Sign]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

    for _ in 0..3 {
        let mut x = 0i128;
        (0..next(&mut it)).for_each(|_| { x += next::<i128>(&mut it); });
        writeln!(so, "{}", match x {
            x if x > 0 => "+",
            x if x < 0 => "-",
            _ => "0",
        }).ok();
    }
}
