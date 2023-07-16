// BOJ 17398 [Network Partitioning]
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
fn union_size(root: &mut Vec<usize>,
              rank: &mut Vec<usize>,
              size: &mut Vec<usize>, x: usize, y: usize) {
    let x = find(root, x);
    let y = find(root, y);
    if x != y {
        if rank[x] < rank[y] {
            root[x] = y;
            size[y] += size[x];
        } else {
            root[y] = x;
            size[x] += size[y];
            if rank[x] == rank[y] {
                rank[x] += 1;
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, q) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let c = (0..m).map(|_| (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1)).collect::<Vec<_>>();
    let d = (0..q).map(|_| next::<usize>(&mut it)-1).collect::<Vec<_>>();
    let mut e = vec![true; m];
    d.iter().for_each(|&i| e[i] = false);

    let mut root = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut size = vec![1; n];
    e.iter().enumerate().filter(|&(_, &b)| b).for_each(|(i, _)| {
        union_size(&mut root, &mut rank, &mut size, c[i].0, c[i].1);
    });

    let mut ans = 0;
    d.iter().rev().for_each(|&i| {
        let (x, y) = c[i];
        let (x, y) = (find(&mut root, x), find(&mut root, y));
        if x != y {
            ans += size[x] * size[y];
            union_size(&mut root, &mut rank, &mut size, x, y);
        }
    });
    println!("{}", ans);
}
