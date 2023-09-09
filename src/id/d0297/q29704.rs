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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, t) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let a = (0..n).map(|_| (
        next::<usize>(&mut it), next::<usize>(&mut it)
    )).collect::<Vec<_>>();

    let mut dp = vec![vec![0; t+1]; n+1];
    let mut ans = 0;
    for i in 0..n {
        for j in 0..t {
            dp[i+1][j] = dp[i+1][j].max(dp[i][j]);
            ans = ans.max(dp[i+1][j]);
            if j + a[i].0 <= t {
                dp[i+1][j+a[i].0] = dp[i+1][j+a[i].0].max(dp[i][j] + a[i].1);
                ans = ans.max(dp[i+1][j+a[i].0]);
            }
        }
    }
    writeln!(so, "{}", a.iter().map(|(_, b)| b).sum::<usize>() - ans).unwrap();
}