// BOJ 2448 [Printing Stars - 11]
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

const UNIT: [[char; 6]; 3] = [
    [' ', ' ', '*', ' ', ' ', ' '],
    [' ', '*', ' ', '*', ' ', ' '],
    ['*', '*', '*', '*', '*', ' ']
    ];
fn get(i: usize, j: usize, m: usize) -> char {
    if m == 3 {
        return UNIT[i][j];
    }
    let n = m/2;
    return if i >= n { get(i % n, j % m, n) }
    else if j < n || j >= n*3 { ' ' }
    else { get(i, (j - n) % m, n) }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n: usize = next(&mut it);
    for i in 0..n {
        for j in 0..2*n-1 {
            write!(so, "{}", get(i, j, n)).ok();
        }
        writeln!(so).ok();
    }
}
