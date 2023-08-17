// BOJ 14177 [Amusement Park]
// Supported by GitHub Copilot

use std::io::{self, Read};
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

fn pref(u: Vec<Vec<i32>>, n: usize) -> Vec<Vec<i32>> {
    let mut p = vec![vec![0; n+1]; n+1];
    for i in 1..=n {
        for j in 1..=n {
            p[i][j] = u[i-1][j-1] + p[i-1][j] + p[i][j-1] - p[i-1][j-1];
        }
    }
    p
}
fn psum(p: &Vec<Vec<i32>>, i: usize, j: usize) -> i32 {
    p[j][j] - p[i-1][j] - p[j][i-1] + p[i-1][i-1]
}

const INF: i32 = 0x3f3f3f3f;
fn dnco(tp: &mut [i32], dp: &[i32], p: &Vec<Vec<i32>>,
        s: usize, e: usize, l: usize, r: usize) {
    if s > e { return; }
    let m = (s+e)>>1;

    let (mut x, mut j) = (INF, 0);
    for i in l..=r.min(m-1) {
        let v = dp[i] + psum(p, i+1, m);
        if x > v { x = v; j = i; }
    }
    if j == 0 { return; }

    tp[m] = x;
    dnco(tp, dp, p, s, m-1, l, j);
    dnco(tp, dp, p, m+1, e, j, r);
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut u = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            u[i][j] = next::<i32>(&mut it);
        }
    }
    let p = pref(u, n);

    let mut dp = vec![0; n+1];
    (1..=n).for_each(|i| dp[i] = psum(&p, 1, i));
    for _ in 1..k {
        let mut tp = vec![0; n+1];
        dnco(&mut tp, &dp, &p, 1, n, 1, n);
        dp = tp;
    }
    println!("{}", dp[n]/2);
}
