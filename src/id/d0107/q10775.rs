// BOJ 10775 [Gates]
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
    } else if x < y {
        root[y] = x;
    } else {
        root[x] = y;
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (g, p) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut root = (0..=g).collect::<Vec<usize>>();
    for i in 0..p {
        let x = find(&mut root, next::<usize>(&mut it));
        if x == 0 {
            println!("{}", i);
            return;
        } else {
            union(&mut root, x-1, x);
        }
    }
    println!("{}", p);
}
