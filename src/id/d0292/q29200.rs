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

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| next::<i64>(&mut it)).collect::<Vec<_>>();
    let k = 4;
    let mut dp = vec![vec![-1; k]; n+1];
    for i in 0..k {
        dp[0][i] = 0;
    }
    for i in 1..=n {
        for j in 0..k {
            if i > j {
                let mut xor = 0;
                for l in 0..=j {
                    xor ^= v[i-1-l];
                }
                for x in 0..k {
                    if x == j { continue; }
                    if dp[i-j-1][x] >= 0 {
                        dp[i][j] = dp[i][j].max(dp[i-j-1][x] + xor);
                    }
                }
            }
        }
    }
    writeln!(so, "{}", dp[n].iter().max().unwrap()).ok();
}