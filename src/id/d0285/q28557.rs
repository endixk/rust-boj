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

    let (a, b) = (
        next::<String>(&mut it).chars().collect::<Vec<_>>(),
        next::<String>(&mut it).chars().collect::<Vec<_>>()
    );
    let (mut p, mut i, mut j) = (0, 0, 0);
    while j < b.len() {
        while i < a.len() && a[i] != b[j] { i += 1; }
        if i == a.len() { writeln!(so, "NO").ok(); return; }
        if (i - p) % 2 == 0 { p = i + 1; i += 1; j += 1; }
        else { i += 1; }
    }
    writeln!(so, "{}", if (a.len() - p) % 2 == 0 { "YES" } else { "NO" }).ok();
}
