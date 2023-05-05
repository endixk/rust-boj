// BOJ 1992 [Quad Tree]
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

fn solve<T>(i: usize, j: usize, x: usize, arr: &Vec<Vec<bool>>, so: &mut T) where T: Write {
    let (res, mut flag) = (arr[i][j], true);
    for k in i..i+x { for l in j..j+x {
        if arr[k][l] != res { flag = false; break }
    }}
    if flag { write!(so, "{}", if res { 1 } else { 0 }).unwrap() }
    else {
        write!(so, "(").unwrap();
        solve(i, j, x/2, arr, so);
        solve(i, j+x/2, x/2, arr, so);
        solve(i+x/2, j, x/2, arr, so);
        solve(i+x/2, j+x/2, x/2, arr, so);
        write!(so, ")").unwrap();
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut arr = vec![vec![false; n]; n];
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            arr[i][j] = c == '1';
        }
    }
    solve(0, 0, n, &arr, &mut so);
}
