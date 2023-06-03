// BOJ 16566 [Card Game]
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

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x {
        x
    } else {
        root[x] = find(root, root[x]);
        root[x]
    }
}

fn union(root: &mut Vec<usize>, x: usize, y: usize) {
    let x = find(root, x);
    let y = find(root, y);
    if x == y {
        return;
    }
    if x < y {
        root[x] = y;
    } else {
        root[y] = x;
    }
}

fn idx(v: &Vec<usize>, x: usize) -> usize {
    match v.binary_search(&(x+1)) {
        Ok(i) => i,
        Err(i) => i,
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut ex = vec![false; n+1];
    (0..m).map(|_| next::<usize>(&mut it)).for_each(|x| ex[x] = true);
    let v = (1..=n).filter(|&x| ex[x]).collect::<Vec<_>>();

    let mut root = (0..=m).collect::<Vec<_>>();
    for _ in 0..k {
        let c = next::<usize>(&mut it);
        let i = idx(&v, c);
        let j = find(&mut root, i);
        write!(so, "{} ", v[j]).unwrap();
        union(&mut root, j, j+1);
    }
    writeln!(so).unwrap();
}
