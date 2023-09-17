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

use std::collections::HashSet;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace().peekable();

    let mut set = HashSet::new();
    for _ in 0..next(&mut it) {
        set.insert(next::<char>(&mut it));
    }
    for _ in 0..next(&mut it) {
        set.insert(next::<char>(&mut it));
    }
    for _ in 0..next(&mut it) {
        let c = next::<char>(&mut it);
        if set.contains(&c) { set.remove(&c); }
    }

    let _ = next::<usize>(&mut it);
    let mut b = 0;
    while it.peek().is_some() {
        let s = next::<String>(&mut it);
        for ch in s.chars() {
            if set.contains(&ch) {
                if b > 0 {writeln!(so, "").ok(); }
                b = 0;
            }
            else {
                write!(so, "{}", ch).ok();
                b += 1;
            }
        }
        if b > 0 {writeln!(so, "").ok(); }
        b = 0;
    }
}