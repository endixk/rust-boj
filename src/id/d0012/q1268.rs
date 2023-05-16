// BOJ 1268 [Temporary Class President]
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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n: usize = next(&mut it);
    let v = (0..n).map(|_|
        (0..5).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    let mut x = vec![vec![false; n]; n];
    for k in 0..5 { for i in 0..n { for j in 0..n {
        if v[i][k] == v[j][k] { x[i][j] = true; }
    }}}

    let (mut ans, mut cnt) = (0, 0);
    (0..n).for_each(|i| {
        let c = (0..n).filter(|&j| x[i][j]).count();
        if cnt < c { ans = i; cnt = c; }
    });
    writeln!(so, "{}", ans+1).ok();
}
