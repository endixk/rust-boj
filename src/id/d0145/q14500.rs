// BOJ 14500 [Tetrominos]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

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

fn i0j3(a: &Vec<Vec<u16>>, i: usize, j: usize) -> u16 {a[i][j] + a[i][j+1] + a[i][j+2] + a[i][j+3]}
fn i3j0(a: &Vec<Vec<u16>>, i: usize, j: usize) -> u16 {a[i][j] + a[i+1][j] + a[i+2][j] + a[i+3][j]}
fn i1j1(a: &Vec<Vec<u16>>, i: usize, j: usize) -> u16 {a[i][j] + a[i][j+1] + a[i+1][j] + a[i+1][j+1]}
fn i1j2(a: &Vec<Vec<u16>>, i: usize, j: usize) -> u16 {
    let mut ret = a[i][j] + a[i][j+1] + a[i][j+2] + a[i+1][j];
    ret = ret.max(a[i][j] + a[i][j+1] + a[i][j+2] + a[i+1][j+1]);
    ret = ret.max(a[i][j] + a[i][j+1] + a[i][j+2] + a[i+1][j+2]);
    ret = ret.max(a[i][j] + a[i+1][j] + a[i+1][j+1] + a[i+1][j+2]);
    ret = ret.max(a[i][j+1] + a[i+1][j] + a[i+1][j+1] + a[i+1][j+2]);
    ret = ret.max(a[i][j+2] + a[i+1][j] + a[i+1][j+1] + a[i+1][j+2]);
    ret = ret.max(a[i][j] + a[i][j+1] + a[i+1][j+1] + a[i+1][j+2]);
    ret = ret.max(a[i][j+1] + a[i][j+2] + a[i+1][j] + a[i+1][j+1]);
    ret
}
fn i2j1(a: &Vec<Vec<u16>>, i: usize, j: usize) -> u16 {
    let mut ret = a[i][j] + a[i+1][j] + a[i+2][j] + a[i][j+1];
    ret = ret.max(a[i][j] + a[i+1][j] + a[i+2][j] + a[i+1][j+1]);
    ret = ret.max(a[i][j] + a[i+1][j] + a[i+2][j] + a[i+2][j+1]);
    ret = ret.max(a[i][j] + a[i][j+1] + a[i+1][j+1] + a[i+2][j+1]);
    ret = ret.max(a[i+1][j] + a[i][j+1] + a[i+1][j+1] + a[i+2][j+1]);
    ret = ret.max(a[i+2][j] + a[i][j+1] + a[i+1][j+1] + a[i+2][j+1]);
    ret = ret.max(a[i][j] + a[i+1][j] + a[i+1][j+1] + a[i+2][j+1]);
    ret = ret.max(a[i][j+1] + a[i+1][j+1] + a[i+1][j] + a[i+2][j]);
    ret
}
fn get(a: &Vec<Vec<u16>>, i: usize, j: usize, n: usize, m: usize) -> u16 {
    let mut ret = 0;
    if j+3 < m {ret = ret.max(i0j3(a, i, j));}
    if i+3 < n {ret = ret.max(i3j0(a, i, j));}
    if i+1 < n && j+1 < m {ret = ret.max(i1j1(a, i, j));}
    if i+1 < n && j+2 < m {ret = ret.max(i1j2(a, i, j));}
    if i+2 < n && j+1 < m {ret = ret.max(i2j1(a, i, j));}
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut a = vec![vec![0u16; m]; n];
    for i in 0..n { for j in 0..m {
        a[i][j] = next::<u16>(&mut it);
    }}

    let mut ans = 0;
    for i in 0..n { for j in 0..m {
        ans = ans.max(get(&a, i, j, n, m));
    }}
    writeln!(so, "{}", ans).ok();
}
