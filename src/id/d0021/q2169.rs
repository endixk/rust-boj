// BOJ 2169 [Robot Control]
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
    let mut a = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            a[i][j] = next::<i32>(&mut it);
        }
    }

    let mut dp = vec![0; m];
    dp[0] = a[0][0];
    for i in 1..m {
        dp[i] = dp[i-1] + a[0][i];
    }
    for i in 1..n {
        let mut lp = vec![0; m];
        lp[0] = dp[0] + a[i][0];
        for j in 1..m {
            lp[j] = a[i][j] + dp[j].max(lp[j-1]);
        }

        let mut rp = vec![0; m];
        rp[m-1] = dp[m-1] + a[i][m-1];
        for j in (0..m-1).rev() {
            rp[j] = a[i][j] + dp[j].max(rp[j+1]);
        }

        dp = lp.iter().zip(rp.iter()).map(|(&x, &y)| x.max(y)).collect();
    }

    writeln!(so, "{}", dp[m-1])?;

    Ok(())
}
