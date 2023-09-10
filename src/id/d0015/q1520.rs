// BOJ 1520 [Going Down The Road Feeling Bad]
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

fn go(adj: &Vec<Vec<usize>>, dp: &mut Vec<i32>, u: usize, dst: usize) -> i32 {
    if u == dst { return 1; }
    if dp[u] != -1 { return dp[u]; }
    dp[u] = 0;
    for &v in adj[u].iter() {
        dp[u] += go(adj, dp, v, dst);
    }
    dp[u]
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (r, c) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut b = vec![vec![0; c]; r];
    let mut adj = vec![vec![]; r * c];
    for i in 0..r { for j in 0..c {
        b[i][j] = next::<usize>(&mut it);
        if i > 0 {
            if b[i][j] > b[i-1][j] {
                adj[i*c+j].push((i-1)*c+j);
            } else if b[i][j] < b[i-1][j] {
                adj[(i-1)*c+j].push(i*c+j);
            }
        }
        if j > 0 {
            if b[i][j] > b[i][j-1] {
                adj[i*c+j].push(i*c+j-1);
            } else if b[i][j] < b[i][j-1] {
                adj[i*c+j-1].push(i*c+j);
            }
        }
    }}

    let mut dp = vec![-1; r * c];
    writeln!(so, "{}", go(&adj, &mut dp, 0, r*c-1)).unwrap();
}
