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

fn go(dp: &mut Vec<Vec<(usize, usize, usize ,usize)>>, w: &[usize], v: &[usize], t: &[usize], k: usize, i: usize) -> (usize, usize, usize, usize) {
    if k == 0 {
        return (w[i], v[i], t[i], 0);
    }
    if dp[k][i].3 != 0 {
        return dp[k][i];
    }
    let (mut ans, mut ai) = (usize::MAX, 0);
    for j in 1..i {
        let (wi, vi, ti, _) = go(dp, w, v, t, k-1, j);
        if ans > w[i] + wi + v[i].max(vi) + t[i].max(ti) {
            ans = w[i] + wi + v[i].max(vi) + t[i].max(ti);
            ai = j;
        }
    }
    dp[k][i] = (w[i] + w[ai], v[i].max(v[ai]), t[i].max(t[ai]), ai);
    dp[k][i]
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let (mut w, mut v, mut t) = (vec![0; n+1], vec![0; n+1], vec![0; n+1]);
    for i in 1..=n {
        w[i] = next::<usize>(&mut it);
        v[i] = next::<usize>(&mut it);
        t[i] = 1_000_000_000 - next::<usize>(&mut it);
    }

    let mut dp = vec![vec![(0, 0, 0, 0); n+1]; k];
    for i in 1..=n {
        go(&mut dp, &w, &v, &t, k-1, i);
    }

    let (mut ans, mut ansi) = (usize::MAX, 0);
    for i in 1..=n {
        if ans > dp[k-1][i].0 + dp[k-1][i].1 + dp[k-1][i].2 && dp[k-1][i].3 > 0 {
            ans = dp[k-1][i].0 + dp[k-1][i].1 + dp[k-1][i].2;
            ansi = i;
        }
    }
    writeln!(so, "{}", dp[k-1][ansi].0 + dp[k-1][ansi].1 + 1_000_000_000 - dp[k-1][ansi].2).ok();
    while ansi > 0 {
        write!(so, "{} ", ansi).ok();
        ansi = dp[k-1][ansi].3;
    }
    writeln!(so, "").ok();
}