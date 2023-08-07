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

use std::collections::HashSet;
fn go(v: &[String]) -> bool {
    for i in 1..v.len() {
        if v[i - 1].chars().last().unwrap() != v[i].chars().next().unwrap() {
            return false;
        }
    }
    true
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = Vec::new();
    let mut set = HashSet::new();
    let mut loc = 0;
    for _ in 0..n {
        let s = next::<String>(&mut it);
        if s == "?" { loc = v.len(); }
        v.push(s.clone());
        set.insert(s);
    }

    for _ in 0..next(&mut it) {
        let a = next::<String>(&mut it);
        if set.contains(&a) { continue; }
        v[loc] = a.clone();
        if go(&v) { writeln!(so, "{a}").ok(); }
    }
}
