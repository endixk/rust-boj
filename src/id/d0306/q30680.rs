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

fn dfs(tree: &Vec<Vec<usize>>, v: &mut Vec<usize>, d: usize, cur: usize, par: usize) {
    let mut f = true;
    for &x in &tree[cur] {
        if x == par { continue; }
        f = false;
        dfs(tree, v, d+1, x, cur);
    }
    if f { v.push(d); }
}
fn depths(tree: Vec<Vec<usize>>) -> Vec<usize> {
    let mut v = vec![];
    dfs(&tree, &mut v, 1, 1, 0);
    v
}

use std::collections::BinaryHeap;
use std::cmp::Reverse;
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut pq = BinaryHeap::new();
    pq.push(Reverse(1));
    for _ in 0..n {
        let a = next::<usize>(&mut it);
        let mut tree = vec![vec![]; a+1];
        for _ in 1..a {
            let (u, v) = (next::<usize>(&mut it), next::<usize>(&mut it));
            tree[u].push(v);
            tree[v].push(u);
        }

        let d = depths(tree);
        let h = pq.pop().unwrap().0;
        for &x in &d {
            pq.push(Reverse(h + x));
        }
    }

    let mut ans = 0;
    while let Some(Reverse(x)) = pq.pop() {
        ans = ans.max(x-1);
    }
    writeln!(so, "{}", ans)?;

    Ok(())
}