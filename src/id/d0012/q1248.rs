// BOJ 1248 [Guess]
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

fn solve(sign: &Vec<Vec<char>>, v: &mut Vec<i8>, x: usize, n: usize) {
    for i in 0..x {
        let s = v[x] - v[i];
        match s {
            s if s > 0 && sign[i+1][x] == '+' => continue,
            s if s < 0 && sign[i+1][x] == '-' => continue,
            s if s == 0 && sign[i+1][x] == '0' => continue,
            _ => return,
        }
    }
    if x == n {
        for i in 0..n {
            print!("{} ", v[i+1] - v[i]);
        }
        std::process::exit(0);
    }
    match sign[x+1][x+1] {
        '+' => for i in 1..=10 {
            v[x+1] = v[x] + i;
            solve(sign, v, x+1, n);
        },
        '-' => for i in -10..=-1 {
            v[x+1] = v[x] + i;
            solve(sign, v, x+1, n);
        },
        '0' => {
            v[x+1] = v[x];
            solve(sign, v, x+1, n);
        },
        _ => unreachable!(),
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n: usize = next(&mut it);
    let s = next::<String>(&mut it);
    let mut sign = vec![vec![' '; n+1]; n+1];
    let mut c = 0;
    for i in 1..=n { for j in i..=n {
        sign[i][j] = s.chars().nth(c).unwrap();
        c += 1;
    }}

    let mut v = vec![0; n+1];
    solve(&sign, &mut v, 0, n);
}
