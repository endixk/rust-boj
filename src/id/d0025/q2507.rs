// BOJ 2507 [Saving the Princess]
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

fn go(v: &[(i32, i32, bool)], dp: &mut Vec<Vec<i32>>, i: usize, j: usize, n: usize) -> i32 {
    if i == n-1 {
        return if v[i].0 - v[i].1 <= v[j].0 { 1 } else { 0 };
    }
    if j == n-1 { return 0; }
    if dp[i][j] != -1 { return dp[i][j]; }

    let mut ret = 0;
    let s = if i < j { j } else { i } + 1;
    for k in s..n {
        if v[i].0 + v[i].1 >= v[k].0 {
            ret = (ret + go(v, dp, k, j, n)) % 1000;
        }
        if v[j].0 >= v[k].0 - v[k].1 && v[k].2 {
            ret = (ret + go(v, dp, i, k, n)) % 1000;
        }
    }

    dp[i][j] = ret;
    dp[i][j]
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut v = vec![(0, 0, false); n];
    for i in 0..n {
        v[i].0 = next::<i32>(&mut it);
        v[i].1 = next::<i32>(&mut it);
        v[i].2 = next::<i32>(&mut it) == 1;
    }
    v.sort_unstable();

    let mut dp = vec![vec![-1; n]; n];
    writeln!(so, "{}", go(&v, &mut dp, 0, 0, n))?;

    Ok(())
}
