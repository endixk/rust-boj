// BOJ 30505 [Secret Friend]
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

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = (0..n).collect::<Vec<_>>();
    let mut b = (0..n).collect::<Vec<_>>();
    for _ in 0..m {
        let (i, j) = (next::<usize>(&mut it), next::<usize>(&mut it));
        a[i-1] = j-1;
        b[j-1] = i-1;
    }

    let q = next::<usize>(&mut it) - 1;
    let c = (0..n).filter(|&i| b[i] == i && i != q).collect::<Vec<_>>();
    if a[q] != q || c.len() == 1 { writeln!(so, "NOJAM")?; }
    else if c.len() == 2 {
        let mut x = q;
        while b[x] != x { x = b[x]; }
        if (c[0] == x && a[c[1]] == c[1]) || (c[1] == x && a[c[0]] == c[0]) {
            writeln!(so, "NOJAM")?;
        } else {
            writeln!(so, "2")?;
        }
    }
    else { writeln!(so, "{}", c.len())?; }

    Ok(())
}
