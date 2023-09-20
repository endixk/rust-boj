// BOJ 16139 [Human-Computer Interaction]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let s = next::<String>(&mut it);
    let mut x = vec![0; 26];
    let mut p = Vec::new();
    p.push(x.clone());
    for c in s.chars() {
        x[c as usize - 'a' as usize] += 1;
        p.push(x.clone());
    }

    for _ in 0..next(&mut it) {
        let (c, l, r) = (next::<char>(&mut it) as usize - 'a' as usize, next::<usize>(&mut it), next::<usize>(&mut it));
        writeln!(so, "{}", p[r+1][c] - p[l][c])?;
    }

    Ok(())
}
