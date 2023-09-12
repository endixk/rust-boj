// BOJ 28278 [Stack 2]
// Supported by GitHub Copilot

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

    let mut st = Vec::new();
    for _ in 0..next(&mut it) {
        match next::<u8>(&mut it) {
            1 => st.push(next::<i32>(&mut it)),
            2 => { writeln!(so, "{}", st.pop().unwrap_or(-1)).ok(); },
            3 => { writeln!(so, "{}", st.len()).ok(); },
            4 => { writeln!(so, "{}", if st.is_empty() { 1 } else { 0 }).ok(); },
            _ => { writeln!(so, "{}", st.last().unwrap_or(&-1)).ok(); },
        }
    }
}
