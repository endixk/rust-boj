// BOJ 1774 [Building Roads]
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let p = (0..n).map(|_| (next::<i64>(&mut it), next::<i64>(&mut it))).collect::<Vec<_>>();

    let mut e = Vec::new();
    for i in 0..n-1 { for j in i+1..n {
        e.push(((p[i].0 - p[j].0).pow(2) + (p[i].1 - p[j].1).pow(2), i, j));
    }}
    e.sort_unstable();

    let mut root = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    for _ in 0..m {
        union(&mut root, &mut rank, next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1);
    }

    let mut ans = 0.0;
    for (d, i, j) in e {
        let (i, j) = (find(&mut root, i), find(&mut root, j));
        if i != j {
            union(&mut root, &mut rank, i, j);
            ans += (d as f64).sqrt();
        }
    }
    writeln!(so, "{:.2}", ans)?;

    Ok(())
}
