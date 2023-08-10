// BOJ 28251 [Nadori Sum]
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
fn union(root: &mut Vec<usize>,
         rank: &mut Vec<usize>,
         sum: &mut Vec<usize>,
         pow: &mut Vec<usize>, x: usize, y: usize) {
    let x = find(root, x);
    let y = find(root, y);
    if x != y {
        if rank[x] < rank[y] {
            root[x] = y;
            pow[y] += pow[x] + sum[x] * sum[y];
            sum[y] += sum[x];
        } else {
            root[y] = x;
            pow[x] += pow[y] + sum[x] * sum[y];
            sum[x] += sum[y];
            if rank[x] == rank[y] {
                rank[x] += 1;
            }
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, q) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut root = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut sum = (0..n).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
    let mut pow = vec![0; n];

    for _ in 0..q {
        let (a, b) = (next::<usize>(&mut it)-1, next::<usize>(&mut it)-1);
        union(&mut root, &mut rank, &mut sum, &mut pow, a, b);
        writeln!(so, "{}", pow[find(&mut root, a)]).ok();
    }
}
