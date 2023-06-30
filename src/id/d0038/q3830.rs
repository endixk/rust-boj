// BOJ 3830 [Never Wait for Weights]
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

fn find(root: &mut Vec<usize>, w: &mut Vec<i32>, x: usize) -> usize {
    if root[x] == x {
        x
    } else {
        let p = find(root, w, root[x]);
        w[x] += w[root[x]];
        root[x] = p;
        root[x]
    }
}

fn union(root: &mut Vec<usize>,
         rank: &mut Vec<usize>,
         w: &mut Vec<i32>, v: i32, x: usize, y: usize) {
    let (i, j) = (find(root, w, x), find(root, w, y));
    if i != j {
        if rank[i] < rank[j] {
            root[i] = j;
            w[i] = w[y] - w[x] - v;
        } else {
            root[j] = i;
            w[j] = w[x] - w[y] + v;
            if rank[i] == rank[j] {
                rank[i] += 1;
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    loop {
        let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
        if n == 0 && m == 0 { break; }

        let mut root = (0..n).collect::<Vec<usize>>();
        let mut rank = vec![0; n];
        let mut w = vec![0; n];
        for _ in 0..m {
            let (a, b, x) = match next(&mut it) {
                '!' => (next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1, Some(next::<i32>(&mut it))),
                '?' => (next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1, None),
                _ => unreachable!()
            };
            if let Some(x) = x {
                union(&mut root, &mut rank, &mut w, x, a, b);
            } else {
                let (i, j) = (find(&mut root, &mut w, a), find(&mut root, &mut w, b));
                writeln!(so, "{}", if i == j { (w[b] - w[a]).to_string() } else { "UNKNOWN".to_string() }).ok();
            }
        }
    }
}
