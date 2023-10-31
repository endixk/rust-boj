// BOJ 2503 [Number Baseball]
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

fn test(x: u16, q: u16, s: u8, t: u8) -> bool {
    let (a, b, c) = (x / 100, x / 10 % 10, x % 10);
    if a == b || a == c || b == c { return false; }
    if a == 0 || b == 0 || c == 0 { return false; }

    let (i, j, k) = (q / 100, q / 10 % 10, q % 10);
    let (x, y) = (
        if a == i { 1 } else { 0 } + if b == j { 1 } else { 0 } + if c == k { 1 } else { 0 },
        if a == j || a == k { 1 } else { 0 } + if b == i || b == k { 1 } else { 0 } + if c == i || c == j { 1 } else { 0 },
    );

    x == s && y == t
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut c = vec![true; 1000];
    for _ in 0..next(&mut it) {
        let (q, s, b) = (next::<u16>(&mut it), next::<u8>(&mut it), next::<u8>(&mut it));
        for x in 0..1000 {
            if !c[x] { continue; }
            if !test(x as u16, q, s, b) { c[x] = false; }
        }
    }
    writeln!(so, "{}", c.iter().filter(|x| **x).count())?;

    Ok(())
}
