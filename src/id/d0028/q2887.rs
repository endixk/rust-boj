// BOJ 2887 [SVEMIR]
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

fn union_size(root: &mut Vec<usize>,
              rank: &mut Vec<usize>,
              size: &mut Vec<usize>, x: usize, y: usize) -> usize {
    let x = find(root, x);
    let y = find(root, y);
    if x != y {
        if rank[x] < rank[y] {
            root[x] = y;
            size[y] += size[x];
            size[y]
        } else {
            root[y] = x;
            size[x] += size[y];
            if rank[x] == rank[y] {
                rank[x] += 1;
            }
            size[x]
        }
    } else {
        size[x]
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let (mut x, mut y, mut z) = (
        Vec::with_capacity(n),
        Vec::with_capacity(n),
        Vec::with_capacity(n));
    (0..n).for_each(|i| {
        x.push((next::<i32>(&mut it), i));
        y.push((next::<i32>(&mut it), i));
        z.push((next::<i32>(&mut it), i));
    });

    x.sort_unstable();
    y.sort_unstable();
    z.sort_unstable();

    let mut d = Vec::with_capacity(n*3);
    for i in 0..n-1 {
        d.push((x[i+1].0 - x[i].0, x[i].1, x[i+1].1));
        d.push((y[i+1].0 - y[i].0, y[i].1, y[i+1].1));
        d.push((z[i+1].0 - z[i].0, z[i].1, z[i+1].1));
    }
    d.sort_unstable();

    let mut root = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut size = vec![1; n];
    let mut ans = 0;
    for (c, a, b) in d {
        if find(&mut root, a) != find(&mut root, b) {
            if union_size(&mut root, &mut rank, &mut size, a, b) == n {
                println!("{}", ans + c);
                return;
            }
            ans += c;
        }
    }
}
