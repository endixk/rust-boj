// BOJ 24426 [Matrix Path 6]
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

    let mut v = Vec::new();
    for _ in 0..next(&mut it) {
        v.push((next::<usize>(&mut it)-1, next::<usize>(&mut it)-1));
    }
    v.sort_unstable();

    let mut ans = go(&mat, 0, 0, v[0].0, v[0].1);
    for i in 1..v.len() {
        if v[i-1].1 > v[i].1 { writeln!(so, "-1")?; return Ok(()); }
        ans += go(&mat, v[i-1].0, v[i-1].1, v[i].0, v[i].1);
    }
    ans += go(&mat, v[v.len()-1].0, v[v.len()-1].1, n-1, n-1);
    for (i, j) in v { ans -= mat[i][j]; }
    writeln!(so, "{}", ans)?;

    Ok(())
}
