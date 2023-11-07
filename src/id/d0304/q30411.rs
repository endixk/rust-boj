// BOJ 30411 [Graph Game]
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

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut e = vec![];
    'a: for i in 1..=n/2 {
        for j in n/2+1..=n {
            e.push((i, j));
            if e.len() == k { break 'a; }
        }
    }

    if e.len() < k { writeln!(so, "NO")?; }
    else {
        writeln!(so, "YES")?;
        for i in 0..k {
            writeln!(so, "{} {}", e[i].0, e[i].1)?;
        }
    }

    Ok(())
}
