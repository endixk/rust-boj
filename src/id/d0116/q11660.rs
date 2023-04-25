// BOJ 11660 [Interval Sum 5]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let (n, m): (usize, usize) = (next(&mut it), next(&mut it));

    let mut a = vec![vec![0; n+1]; n+1];
    for i in 1..=n {
        for j in 1..=n {
            let x: i32 = next(&mut it);
            a[i][j] = a[i-1][j] + a[i][j-1] - a[i-1][j-1] + x;
        }
    }

    for _ in 0..m {
        let (x1, y1, x2, y2): (usize, usize, usize, usize) = (
            next(&mut it),
            next(&mut it),
            next(&mut it),
            next(&mut it)
        );
        writeln!(so, "{}", a[x2][y2] - a[x1-1][y2] - a[x2][y1-1] + a[x1-1][y1-1]).unwrap();
    }
}
