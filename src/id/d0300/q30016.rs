// BOJ 30016 [Study Scheduling]
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

    let (n, t) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut s = vec![vec![0; t+1]; n];
    for i in 0..n { for j in 0..=t {
        s[i][j] = next::<i32>(&mut it);
    }}

    let mut dp = vec![vec![0; t+1]; n+1];
    let mut tr = vec![vec![0; t+1]; n+1];
    for i in 0..n {
        for j in 0..=t {
            for k in 0..=t {
                if t < j+k { break; }
                if dp[i+1][j+k] < dp[i][j] + s[i][k] {
                    dp[i+1][j+k] = dp[i][j] + s[i][k];
                    tr[i+1][j+k] = k;
                }
            }
        }
    }

    let (mut ans, mut at) = (-0x3f3f3f3f, 0);
    for i in 0..=t {
        let d = next::<i32>(&mut it);
        if ans < dp[n][i] - d {
            ans = dp[n][i] - d;
            at = i;
        }
    }
    let mut v = Vec::new();
    for i in (0..n).rev() {
        v.push(tr[i+1][at]);
        at -= tr[i+1][at];
    }

    writeln!(so, "{}", ans)?;
    for &x in v.iter().rev() { write!(so, "{} ", x)?; }
    writeln!(so)?;

    Ok(())
}
