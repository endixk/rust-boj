// BOJ 1866 [Delivery]
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

fn helo(p: &[usize], a: &[usize], i: usize, j: usize, h: usize, t: usize) -> usize {
    if i == j { return h; }
    if i == j-1 { return h + t * (a[i] - a[i-1]); }
    let m = (i+j)/2;
    return if (j-i)%2 == 0 { h + t * (p[j] + p[i-1] - p[m] - p[m-1]) }
    else { h + t * (p[j] + p[i-1] - 2*p[m]) };
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut a = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let (t, h) = (next::<usize>(&mut it), next::<usize>(&mut it));

    a.sort_unstable();
    let mut p = vec![0; n+1];
    for i in 0..n { p[i+1] = p[i] + a[i]; }

    let mut dp = vec![0; n+1];
    for j in 1..=n {
        dp[j] = dp[j-1] + t * a[j-1];
        for i in 1..=j {
            dp[j] = dp[j].min(dp[i-1] + helo(&p, &a, i, j, h, t));
        }
    }
    writeln!(so, "{}", dp[n]).unwrap();
}
