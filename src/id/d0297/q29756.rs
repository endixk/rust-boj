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

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let s = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let h = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();

    let mut dp = vec![vec![-1; 101]; n+1];
    dp[0][100] = 0;
    for i in 0..n {
        for j in 0..=100 {
            if dp[i][j] == -1 { continue; }
            if j >= h[i] {
                let nxt = if j - h[i] + k > 100 { 100 } else { j - h[i] + k };
                dp[i+1][nxt] = dp[i+1][nxt].max(dp[i][j] + s[i]);
            }
            let nxt = if j + k > 100 { 100 } else { j + k };
            dp[i+1][nxt] = dp[i+1][nxt].max(dp[i][j]);
        }
    }

    writeln!(so, "{}", dp[n].iter().max().unwrap()).unwrap();
}