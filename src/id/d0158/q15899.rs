// BOJ 15899 [Tree and Colors]
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

struct MergeSortTree<T> { n: usize, v: Vec<Vec<T>> }
impl<T> MergeSortTree<T> where T: Ord + Default + Copy {
    fn merge(a: &[T], b: &[T]) -> Vec<T> {
        let (mut i, mut j) = (0, 0);
        let (m, n) = (a.len(), b.len());
        let mut c = Vec::with_capacity(m + n);
        while i < m && j < n {
            if a[i] < b[j] { c.push(a[i]); i += 1; }
            else { c.push(b[j]); j += 1; }
        }
        while i < m { c.push(a[i]); i += 1; }
        while j < n { c.push(b[j]); j += 1; }
        c
    }
    fn new(n: usize, a: &[T]) -> Self {
        let mut m = 1;
        while m < n { m <<= 1; }
        let mut v = vec![vec![]; m<<1];
        for i in 0..n { v[i+m] = vec![a[i]]; }
        for i in (1..m).rev() { v[i] = Self::merge(&v[i<<1], &v[i<<1|1]); }
        Self { n: m, v }
    }
    fn query(&self, mut l: usize, mut r: usize, x: T, y: T) -> usize {
        if l > r { return 0; }
        if x > y { return 0; }
        l += self.n; r += self.n;
        let mut ans = 0;
        while l <= r {
            if l & 1 == 1 {
                ans += self.v[l].partition_point(|&z| z <= y) - self.v[l].partition_point(|&z| z < x);
                l += 1;
            }
            if r & 1 == 0 {
                ans += self.v[r].partition_point(|&z| z <= y) - self.v[r].partition_point(|&z| z < x);
                r -= 1;
            }
            l >>= 1; r >>= 1;
        }
        ans
    }
}

fn ett(adj: &Vec<Vec<usize>>, sz: &mut Vec<usize>, map: &mut Vec<usize>, cur: usize, par: usize,
       a: &Vec<i32>, b: &mut Vec<i32>, c: &mut usize) -> usize {
    sz[cur] = 1; map[cur] = *c;
    b[*c] = a[cur]; *c += 1;
    for &nxt in &adj[cur] {
        if nxt == par { continue; }
        sz[cur] += ett(adj, sz, map, nxt, cur, a, b, c);
    }
    sz[cur]
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m, _) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<i32>(&mut it));
    let a = (0..n).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let mut adj = vec![vec![]; n];
    for _ in 1..n {
        let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u-1].push(v-1);
        adj[v-1].push(u-1);
    }

    let mut sz = vec![0; n];
    let mut map = vec![0; n];
    let mut b = vec![0; n];
    ett(&adj, &mut sz, &mut map, 0, 0, &a, &mut b, &mut 0);
    let mst = MergeSortTree::new(n, &b);

    let mut ans = 0;
    for _ in 0..m {
        let (v, c) = (next::<usize>(&mut it), next::<i32>(&mut it));
        let (l, r) = (map[v-1], map[v-1]+sz[v-1]-1);
        ans = (ans + mst.query(l, r, 0, c)) % 1000000007;
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}
