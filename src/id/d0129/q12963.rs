// BOJ 12963 [Running]
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

const MOD: usize = 1_000_000_007;
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<usize>(&mut it), next::<usize>(&mut it));
    let mut rem = vec![1];
    for i in 1..m {
        rem.push(rem[i-1] * 3 % MOD);
    }

    let e = (0..m).map(|_| (next::<usize>(&mut it), next::<usize>(&mut it))).collect::<Vec<_>>();
    let mut root = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut ans = 0;
    for (i, &(x, y)) in e.iter().enumerate().rev() {
        let (s, e, p, q) = (
            find(&mut root, 0),
            find(&mut root, n-1),
            find(&mut root, x),
            find(&mut root, y)
        );
        if (s == p && e == q) || (s == q && e == p) {
            ans += rem[i];
            ans %= MOD;
        } else {
            union(&mut root, &mut rank, x, y);
        }
    }
    println!("{}", ans);
}
