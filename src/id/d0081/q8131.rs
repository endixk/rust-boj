// BOJ 8131 [Ploughing]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

fn query_row(rps: &Vec<Vec<usize>>, i: usize, j1: usize, j2: usize) -> usize {
    rps[i][j2+1] - rps[i][j1] // sum[j1, j2]
}
fn query_col(cps: &Vec<Vec<usize>>, i1: usize, i2: usize, j: usize) -> usize {
    cps[i2+1][j] - cps[i1][j] // sum[i1, i2]
}
fn solve(field: &Vec<Vec<usize>>, n: usize, m: usize, k: usize) -> usize {
    // precompute prefix sum
    let mut rps = vec![vec![0; m+1]; n+1]; // row prefix sum
    let mut cps = vec![vec![0; m+1]; n+1]; // column prefix sum
    for i in 0..n { for j in 0.. m {
        rps[i+1][j+1] = rps[i+1][j] + field[i][j];
        cps[i+1][j+1] = cps[i][j+1] + field[i][j];
    }}

    // precompute ploughable area
    let mut plo = vec![vec![0; m+1]; n];
    for p in 0..n {
        // two-pointer
        let (mut q, mut r) = (0, 0);
        while q < m {
            while r < m && query_row(&rps, p, q, r) <= k {
                r += 1;
            }
            plo[p][q] = r;
            q += 1;
        }
    }

    println!("{:?}", plo);
    0
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    // read input
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (k, m, n) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut field = vec![vec![0; m]; n];
    for i in 0..n { for j in 0.. m {
        field[i][j] = next::<usize>(&mut it);
    }}

    // rotate field
    let mut field_t = vec![vec![0; n]; m];
    for i in 0..n { for j in 0.. m {
        field_t[j][i] = field[i][j];
    }}

    // solve
    writeln!(so, "{}", solve(&field, n, m, k).max(solve(&field_t, m, n, k))).ok();
}
