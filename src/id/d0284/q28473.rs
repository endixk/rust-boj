// BOJ 28473 [Road Signs]
// Supported by GitHub Copilot

use std::io::{self, Read};
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
fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    if rank[x] < rank[y] {
        root[x] = y;
    } else {
        root[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}

use std::collections::BinaryHeap;
use std::cmp::Reverse;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut pq = BinaryHeap::new();
    for _ in 0..m {
        let (x, y, z, w) = (
            next::<usize>(&mut it)-1,
            next::<usize>(&mut it)-1,
            next::<u8>(&mut it),
            next::<u64>(&mut it),
        );
        pq.push(Reverse((z, w, x, y)));
    }

    let mut root = (0..n).collect::<Vec<usize>>();
    let mut rank = vec![0; n];
    let mut cost = 0;
    let mut digits = Vec::new();
    let mut cnt = 0;
    while let Some(Reverse((z, w, x, y))) = pq.pop() {
        let (x, y) = (find(&mut root, x), find(&mut root, y));
        if x != y {
            union(&mut root, &mut rank, x, y);
            cost += w;
            digits.push(z);
            cnt += 1;
        }
    }

    if cnt != n-1 {
        println!("-1");
        return;
    }
    digits.sort_unstable();
    println!("{} {}", digits.iter().map(|&x| (b'0' + x) as char).collect::<String>(), cost);
}
