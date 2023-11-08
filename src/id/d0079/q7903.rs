// BOJ 7903 [Pearls]
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

const INF: usize = 0x3f3f3f3f;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    for _ in 0..next(&mut it) {
        let n = next(&mut it);
        let (mut a, mut p) = (vec![], vec![]);
        for _ in 0..n {
            a.push(next::<usize>(&mut it));
            p.push(next::<usize>(&mut it));
        }

        let mut dp = vec![INF; n+1];
        dp[0] = 0;
        for i in 1..=n {
            for j in 0..i {
                let s = a[j..i].iter().sum::<usize>();
                dp[i] = dp[i].min(dp[j] + (s + 10) * p[i-1]);
            }
        }
        writeln!(so, "{}", dp[n])?;
    }

    Ok(())
}
