use std::io::{self, Read, Write};
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

use std::collections::BinaryHeap;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let p = (0..n).map(|_| (next::<i64>(&mut it), next::<i64>(&mut it))).collect::<Vec<_>>();
    let mut pq = BinaryHeap::new();
    for i in 0..n-1 {
        for j in i+1..n {
            pq.push((-((p[i].0 - p[j].0) * (p[i].0 - p[j].0) + (p[i].1 - p[j].1) * (p[i].1 - p[j].1)), i, j));
        }
    }

    let mut root = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut cnt = 0;
    while cnt < n-1 {
        let (_, i, j) = pq.pop().unwrap();
        if find(&mut root, i) != find(&mut root, j) {
            union(&mut root, &mut rank, i, j);
            writeln!(so, "{} {}", i+1, j+1).unwrap();
            cnt += 1;
        }
    }
}