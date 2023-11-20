// BOJ 30504 [Debt]
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

    let n = next::<usize>(&mut it);
    let mut a = (0..n).map(|i| (next::<i32>(&mut it), i)).collect::<Vec<_>>();
    let mut b = (0..n).map(|i| (next::<i32>(&mut it), i)).collect::<Vec<_>>();
    a.sort_unstable();
    b.sort_unstable();

    let mut c = vec![0; n];
    for ((x, i), (y, _)) in a.into_iter().zip(b.into_iter()) {
        if y < x {
            writeln!(so, "-1")?;
            return Ok(());
        }
        c[i] = y;
    }

    for i in 0..n {
        write!(so, "{} ", c[i])?;
    }
    writeln!(so)?;

    Ok(())
}
