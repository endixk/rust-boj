// BOJ 25953 [Temporal Graph]
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

    let (n, t, m) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let (s, e) = (next::<usize>(&mut it), next::<usize>(&mut it));

    const INF: i32 = 0x3f3f3f3f;
    let mut dp = vec![INF; n];
    dp[s] = 0;
    for _ in 0..t {
        let mut tp = dp.clone();
        for _ in 0..m {
            let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i32>(&mut it));
            if dp[u] != INF {
                tp[v] = tp[v].min(dp[u] + w);
            }
            if dp[v] != INF {
                tp[u] = tp[u].min(dp[v] + w);
            }
        }
        dp = tp;
    }

    writeln!(so, "{}", if dp[e] == INF { -1 } else { dp[e] })?;

    Ok(())
}
