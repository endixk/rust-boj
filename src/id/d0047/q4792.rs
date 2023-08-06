// BOJ 4792 [Red/Blue Spanning Tree]
// Supported by GitHub Copilot

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

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    loop {
        let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        if n+m+k == 0 { break; }

        let (mut r, mut b) = (Vec::new(), Vec::new());
        for _ in 0..m {
            match next::<char>(&mut it) {
                'R' => r.push((next::<usize>(&mut it)-1, next::<usize>(&mut it)-1)),
                _ => b.push((next::<usize>(&mut it)-1, next::<usize>(&mut it)-1)),
            }
        }

        // sweep red edges
        let mut root = (0..n).collect::<Vec<usize>>();
        let mut rank = vec![0; n];
        for &(i, j) in &r {
            union(&mut root, &mut rank, i, j);
        }

        // sweep blue edges
        let mut cnt = 0;
        let mut used = vec![false; b.len()];
        for (i, &(x, y)) in b.iter().enumerate() {
            let (x, y) = (find(&mut root, x), find(&mut root, y));
            if x != y {
                union(&mut root, &mut rank, x, y);
                used[i] = true;
                cnt += 1;
            }
        }
        if cnt > k {
            writeln!(so, "0").unwrap();
            continue;
        }

        // sweep blue edges
        root = (0..n).collect::<Vec<usize>>();
        rank = vec![0; n];
        for (&(x, y), &u) in b.iter().zip(used.iter()) {
            if u { union(&mut root, &mut rank, x, y); }
            else {
                let (x, y) = (find(&mut root, x), find(&mut root, y));
                if x != y {
                    union(&mut root, &mut rank, x, y);
                    cnt += 1;
                }
            }
        }
        writeln!(so, "{}", if cnt < k { 0 } else { 1 }).unwrap();
    }
}
