// BOJ 24426 [Matrix Path 3]
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

fn go(mat: &Vec<Vec<i64>>, si: usize, sj: usize, ei: usize, ej: usize) -> i64 {
    let mut dp = vec![vec![0; ej-sj+1]; ei-si+1];
    dp[0][0] = mat[si][sj];
    for i in si..ei { dp[i-si+1][0] = mat[i+1][sj] + dp[i-si][0]; }
    for j in sj..ej { dp[0][j-sj+1] = mat[si][j+1] + dp[0][j-sj]; }
    for i in si..ei { for j in sj..ej {
        dp[i-si+1][j-sj+1] = mat[i+1][j+1] + dp[i-si][j-sj+1].max(dp[i-si+1][j-sj]);
    }}
    dp[ei-si][ej-sj]
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut mat = vec![vec![0; n]; n];
    for i in 0..n { for j in 0..n {
        mat[i][j] = next::<i64>(&mut it);
    }}
    let (i, j) = (next::<usize>(&mut it), next::<usize>(&mut it));

    let x = go(&mat, 0, 0, i-1, j-1) + go(&mat, i-1, j-1, n-1, n-1) - mat[i-1][j-1];
    mat[i-1][j-1] = -(1<<40);
    let y = go(&mat, 0, 0, n-1, n-1);
    writeln!(so, "{} {}", x, y)?;

    Ok(())
}
