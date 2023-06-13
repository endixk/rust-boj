// BOJ 6581 [HTML]
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

    let mut buf = String::new();
    for word in s.split_ascii_whitespace() {
        if word == "<br>" {
            writeln!(so, "{}", buf).ok();
            buf.clear();
        } else if word == "<hr>" {
            if !buf.is_empty() { writeln!(so, "{}", buf).ok(); }
            writeln!(so, "{}", "-".repeat(80)).ok();
            buf.clear();
        } else {
            if buf.is_empty() { buf.push_str(word); }
            else if buf.len() + word.len() < 80 { buf.push(' '); buf.push_str(word); }
            else { writeln!(so, "{}", buf).ok(); buf.clear(); buf.push_str(word); }
        }
    }
    if !buf.is_empty() { writeln!(so, "{}", buf).ok(); }
}
