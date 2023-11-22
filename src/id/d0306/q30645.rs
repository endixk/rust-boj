// BOJ 30645 [Display Dolls]
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

    let (r, c, n) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut h = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    h.sort_unstable();

    let mut b = vec![vec![0; c]; r+1];
    let mut i = 0;
    let mut ans = 0;
    'a: for x in 1..=r { for y in 0..c {
        while i < n && h[i] <= b[x-1][y] { i += 1; }
        if i == n { break 'a; }
        b[x][y] = h[i];
        ans += 1;
        i += 1;
    }}
    writeln!(so, "{}", ans)?;

    Ok(())
}
