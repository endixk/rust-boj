// BOJ 17132 [Mole]
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
fn union_sz(root: &mut Vec<usize>,
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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut e = (1..n).map(|_|
        (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1, next::<usize>(&mut it))
    ).collect::<Vec<_>>();
    e.sort_unstable_by(|a, b| b.2.cmp(&a.2));

    let mut root = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut size = vec![1; n];
    let mut ans = 0;
    for (u, v, w) in e {
        let (u, v) = (find(&mut root, u), find(&mut root, v));
        if u != v {
            ans += w * size[u] * size[v];
            union_sz(&mut root, &mut rank, &mut size, u, v);
        }
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
