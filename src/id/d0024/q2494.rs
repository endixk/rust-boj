// BOJ 2494 [Matching Numbers]
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

    let mut a = vec![0; n+1];
    let s = next::<String>(&mut it);
    for (j, c) in s.chars().enumerate() {
        a[j+1] = c.to_digit(10).unwrap() as usize;
    }

    let mut b = vec![0; n+1];
    let s = next::<String>(&mut it);
    for (j, c) in s.chars().enumerate() {
        b[j+1] = c.to_digit(10).unwrap() as usize;
    }

    const INF: usize = 111111;
    let mut dp = vec![vec![INF; 10]; n+1];
    let mut bx = vec![vec![0; 10]; n+1];
    let mut bl = vec![vec![false; 10]; n+1];
    dp[0][0] = 0;

    for i in 1..=n {
        for j in 0..10 {
            if dp[i-1][j] == INF { continue; }
            let x = (a[i] + j) % 10;

            let l = (b[i] + 10 - x) % 10;
            let k = (j + l) % 10;
            if dp[i][k] > dp[i-1][j] + l {
                dp[i][k] = dp[i-1][j] + l;
                bx[i][k] = j;
                bl[i][k] = true;
            }

            let r = (x + 10 - b[i]) % 10;
            let j = j;
            if dp[i][j] > dp[i-1][j] + r {
                dp[i][j] = dp[i-1][j] + r;
                bx[i][j] = j;
                bl[i][j] = false;
            }
        }
    }

    let ans = *dp[n].iter().min().unwrap();
    let mut k = dp[n].iter().position(|&x| x == ans).unwrap();
    let mut trk = vec![];
    for i in (1..=n).rev() {
        let (x, l) = (bx[i][k], bl[i][k]);
        let x = (a[i] + x) % 10;
        let r = if l { (b[i] + 10 - x) % 10 } else { (x + 10 - b[i]) % 10 } as i32;
        trk.push(if l { r } else { -r });
        k = bx[i][k];
    }

    writeln!(so, "{}", ans)?;
    for (i, &x) in trk.iter().rev().enumerate() {
        writeln!(so, "{} {}", i+1, x)?;
    }

    Ok(())
}
