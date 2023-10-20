// BOJ 17128 [Cows]
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

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = a[i] * a[(i + 1) % n] * a[(i + 2) % n] * a[(i + 3) % n];
    }
    let mut s = b.iter().sum::<i32>();

    for _ in 0..q {
        let x = next::<usize>(&mut it) - 1;
        for i in 0..4 {
            let j = (x + n - i) % n;
            s -= b[j] * 2;
            b[j] = -b[j];
        }
        writeln!(so, "{}", s)?;
    }

    Ok(())
}
