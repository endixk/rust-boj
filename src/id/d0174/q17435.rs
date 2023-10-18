// BOJ 17435 [Composite Function and Queries]
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

    let m = next::<usize>(&mut it);
    let mut f = vec![vec![0; m+1]; 19];
    for i in 1..=m {
        f[0][i] = next::<usize>(&mut it);
    }
    for p in 1..19 {
        for i in 1..=m {
            f[p][i] = f[p-1][f[p-1][i]];
        }
    }

    for _ in 0..next(&mut it) {
        let (n, mut x) = (next::<usize>(&mut it), next::<usize>(&mut it));
        for i in 0..19 {
            if n & (1 << i) != 0 {
                x = f[i][x];
            }
        }
        writeln!(so, "{}", x)?;
    }

    Ok(())
}
