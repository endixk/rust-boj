// BOJ 30648 [Trick Flower]
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

    let (mut a, mut b, r) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut v = vec![vec![false; r+1]; r+1];

    let mut ans = 0;
    while !v[a][b] {
        v[a][b] = true;
        (a, b) = if a + b + 2 < r {
            (a + 1, b + 1)
        } else {
            (a / 2, b / 2)
        };
        ans += 1;
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
