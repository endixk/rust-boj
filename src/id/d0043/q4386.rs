// BOJ 4386 [Freckles]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;

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

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug)]
struct sf64(f64);
impl Eq for sf64 {}
impl PartialEq for sf64 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Ord for sf64 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 < other.0 { Ordering::Less }
        else if self.0 > other.0 { Ordering::Greater }
        else { Ordering::Equal }
    }
}
impl PartialOrd for sf64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x {
        x
    } else {
        root[x] = find(root, root[x]);
        root[x]
    }
}

fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let x = find(root, x);
    let y = find(root, y);
    if x == y {
        return;
    }
    if rank[x] < rank[y] {
        root[x] = y;
    } else {
        root[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let v = (0..n).map(|_| {
        let x = next::<f64>(&mut it);
        let y = next::<f64>(&mut it);
        (x, y)
    }).collect::<Vec<_>>();

    let mut edges = Vec::new();
    for i in 0..n-1 { for j in i+1..n {
        let (x1, y1) = v[i];
        let (x2, y2) = v[j];
        let d = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
        edges.push((sf64(d), i, j));
    }}
    edges.sort_unstable();

    let mut root = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut ans = 0.0;
    for (d, i, j) in edges {
        if find(&mut root, i) != find(&mut root, j) {
            union(&mut root, &mut rank, i, j);
            ans += d.0;
        }
    }

    println!("{:.2}", ans);
}
