// BOJ 30205 [Mission]
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

    let (n, m, mut p) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i64>(&mut it));
    for _ in 0..n {
        let mut a = Vec::new();
        let mut c = 0;
        for _ in 0..m {
            let x = next::<i64>(&mut it);
            if x < 0 { c += 1; }
            else { a.push(x); }
        }

        a.sort_unstable();
        let mut i = 0;
        loop {
            if i == a.len() { break; }
            if a[i] <= p { p += a[i]; i += 1; }
            else if c > 0 { p <<= 1; c -= 1; }
            else {
                writeln!(so, "0")?;
                return Ok(());
            }
        }
        while c > 0 { p <<= 1; c -= 1; }
    }

    writeln!(so, "1")?;

    Ok(())
}
