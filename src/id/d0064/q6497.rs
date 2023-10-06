// BOJ 6497 [Dark Roads]
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

    loop {
        let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
        if n + m == 0 { break; }

        let mut ans = 0;
        let mut e = Vec::new();
        for _ in 0..m {
            let (u, v, w) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
            e.push((w, u, v));
            ans += w;
        }
        e.sort_unstable();

        let mut root = (0..n).collect::<Vec<usize>>();
        let mut rank = vec![0; n];
        for (w, u, v) in e {
            let (u, v) = (find(&mut root, u), find(&mut root, v));
            if u != v {
                union(&mut root, &mut rank, u, v);
                ans -= w;
            }
        }

        writeln!(so, "{}", ans)?;
    }

    Ok(())
}
