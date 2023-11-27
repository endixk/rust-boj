// BOJ 30777 [DDR]
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

#[inline] fn loc(c: char) -> usize {
    match c { 'U' => 1, 'D' => 2, 'L' => 3, 'R' => 4, _ => 0 }
}
#[inline] fn dist(i: usize, j: usize) -> f64 {
    if i > j { dist(j, i) }
    else if i == j { 0.0 }
    else if i == 0 { 1.0 }
    else if i == 1 {
        if j == 2 { 2.0 } else { 2.0f64.sqrt() }
    }
    else if i == 2 { 2.0f64.sqrt() }
    else { 2.0 }
}
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    const INF: f64 = 1e9;
    let n = next::<usize>(&mut it);
    let mut dp = vec![vec![INF; 5]; 5];
    dp[0][0] = 0.0;
    for _ in 0..n {
        let mut tp = vec![vec![INF; 5]; 5];
        if next::<usize>(&mut it) == 1 {
            let c = loc(next::<char>(&mut it));
            for i in 0..5 { for j in 0..5 {
                tp[c][j] = tp[c][j].min(dp[i][j] + dist(i, c));
                tp[i][c] = tp[i][c].min(dp[i][j] + dist(j, c));
            }}
        }
        else {
            let (c, d) = (loc(next::<char>(&mut it)), loc(next::<char>(&mut it)));
            for i in 0..5 { for j in 0..5 {
                tp[c][d] = tp[c][d].min(dp[i][j] + dist(i, c) + dist(j, d));
                tp[d][c] = tp[d][c].min(dp[i][j] + dist(i, d) + dist(j, c));
            }}
        }
        dp = tp;
    }

    writeln!(so, "{:.10}", dp.into_iter().flatten().reduce(f64::min).unwrap())?;

    Ok(())
}
