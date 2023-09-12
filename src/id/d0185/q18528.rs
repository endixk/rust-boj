// BOJ 18528 [Queue 2]
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

    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    for _ in 0..next(&mut it) {
        match next::<String>(&mut it).as_str() {
            "push" => { q.push_back(next::<i32>(&mut it)); },
            "pop" => { writeln!(so, "{}", q.pop_front().unwrap_or(-1)).ok(); },
            "size" => { writeln!(so, "{}", q.len()).ok(); },
            "empty" => { writeln!(so, "{}", if q.is_empty() { 1 } else { 0 }).ok(); },
            "front" => { writeln!(so, "{}", q.front().unwrap_or(&-1)).ok(); },
            "back" => { writeln!(so, "{}", q.back().unwrap_or(&-1)).ok(); },
            _ => { unreachable!(); },
        }
    }
}
