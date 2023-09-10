// BOJ 2578 [Bingo]
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

fn scan(b: &Vec<Vec<bool>>) -> u8 {
    let mut ret = 0;
    for i in 0..5 {
        ret += if b[i].iter().all(|&x| x) { 1 } else { 0 };
        ret += if (0..5).all(|j| b[j][i]) { 1 } else { 0 };
    }
    ret += if (0..5).all(|i| b[i][i]) { 1 } else { 0 };
    ret += if (0..5).all(|i| b[i][4 - i]) { 1 } else { 0 };
    ret
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut v = vec![(0, 0); 25];
    for i in 0..5 { for j in 0..5 {
        v[next::<usize>(&mut it) - 1] = (i, j);
    }}

    let mut b = vec![vec![false; 5]; 5];
    for i in 0..25 {
        let (x, y) = v[next::<usize>(&mut it) - 1];
        b[x][y] = true;
        if scan(&b) >= 3 {
            writeln!(so, "{}", i + 1).unwrap();
            break;
        }
    }
}
