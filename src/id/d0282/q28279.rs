// BOJ 28279 [Deque 2]
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

    let mut q = std::collections::VecDeque::<i32>::new();
    for _ in 0..next(&mut it) {
        match next::<u8>(&mut it) {
            1 => q.push_front(next(&mut it)),
            2 => q.push_back(next(&mut it)),
            3 => writeln!(so, "{}", q.pop_front().unwrap_or(-1)).unwrap(),
            4 => writeln!(so, "{}", q.pop_back().unwrap_or(-1)).unwrap(),
            5 => writeln!(so, "{}", q.len()).unwrap(),
            6 => writeln!(so, "{}", if q.is_empty() { 1 } else { 0 }).unwrap(),
            7 => writeln!(so, "{}", q.front().unwrap_or(&-1)).unwrap(),
            _ => writeln!(so, "{}", q.back().unwrap_or(&-1)).unwrap(),
        }
    }
}
