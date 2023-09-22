// BOJ 24427 [Matrix Path 4]
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
    let mut mat = vec![vec![0; n]; n];
    for i in 0..n { for j in 0..n {
        mat[i][j] = next::<u32>(&mut it);
    }}

    let mut dpf = vec![vec![0; n]; n];
    dpf[0][0] = mat[0][0];
    for i in 1..n { dpf[i][0] = dpf[i-1][0] + mat[i][0]; }
    for j in 1..n { dpf[0][j] = dpf[0][j-1] + mat[0][j]; }
    for i in 1..n { for j in 1..n {
        dpf[i][j] = mat[i][j] + dpf[i-1][j].max(dpf[i][j-1]);
    }}

    let mut dpr = vec![vec![0; n]; n];
    dpr[n-1][n-1] = mat[n-1][n-1];
    for i in (0..n-1).rev() { dpr[i][n-1] = dpr[i+1][n-1] + mat[i][n-1]; }
    for j in (0..n-1).rev() { dpr[n-1][j] = dpr[n-1][j+1] + mat[n-1][j]; }
    for i in (0..n-1).rev() { for j in (0..n-1).rev() {
        dpr[i][j] = mat[i][j] + dpr[i+1][j].max(dpr[i][j+1]);
    }}

    let mut ans = 0;
    for _ in 0..next(&mut it) {
        let (i, j) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1);
        ans = ans.max(dpf[i][j] + dpr[i][j] - mat[i][j]);
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
