// BOJ 1780 [Number of Papers]
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

static mut C: (usize, usize, usize) = (0, 0, 0);
fn query(p: &Vec<Vec<i16>>, i1: usize, i2: usize, j1: usize, j2: usize) -> i16 {
    p[i2+1][j2+1] - p[i1][j2+1] - p[i2+1][j1] + p[i1][j1]
}
fn solve(plz: &Vec<Vec<i16>>, pez: &Vec<Vec<i16>>, pgz: &Vec<Vec<i16>>, x: usize, y: usize, k: usize) {
    let sz = (k*k) as i16;
    if query(plz, x, x+k-1, y, y+k-1) == sz { unsafe { C.0 += 1 }; return }
    if query(pez, x, x+k-1, y, y+k-1) == sz { unsafe { C.1 += 1 }; return }
    if query(pgz, x, x+k-1, y, y+k-1) == sz { unsafe { C.2 += 1 }; return }
    for i in 0..3 { for j in 0..3 {
        solve(plz, pez, pgz, x+i*k/3, y+j*k/3, k/3);
    }}
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n = next::<usize>(&mut it);

    let mut plz = vec![vec![0i16; n+1]; n+1];
    let mut pez = vec![vec![0i16; n+1]; n+1];
    let mut pgz = vec![vec![0i16; n+1]; n+1];
    for i in 0..n { for j in 0..n {
        let x = next::<i8>(&mut it);
        plz[i+1][j+1] = plz[i+1][j] + plz[i][j+1] - plz[i][j] + if x < 0 { 1 } else { 0 };
        pez[i+1][j+1] = pez[i+1][j] + pez[i][j+1] - pez[i][j] + if x == 0 { 1 } else { 0 };
        pgz[i+1][j+1] = pgz[i+1][j] + pgz[i][j+1] - pgz[i][j] + if x > 0 { 1 } else { 0 };
    }}

    solve(&plz, &pez, &pgz, 0, 0, n);
    unsafe {
        println!("{} {} {}", C.0, C.1, C.2);
    }
}
