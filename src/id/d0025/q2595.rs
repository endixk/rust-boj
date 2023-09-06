// BOJ 2595 [Multiple]
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

use std::collections::VecDeque;
fn bfs(adj: &Vec<Vec<usize>>, digits: &[usize], n: usize) -> Option<Vec<u8>> {
    let mut pu = vec![n; n];
    let mut pd = vec![n; n];
    let mut q = VecDeque::new();
    for &d in digits {
        if d == 0 { continue; }
        pu[d] = 0; pd[d] = d;
        q.push_back(d);
    }
    while let Some(x) = q.pop_front() {
        for &d in digits {
            if adj[x][d] == 0 {
                let mut ret = vec![d as u8];
                let mut cur = x;
                while cur != 0 {
                    ret.push(pd[cur] as u8);
                    cur = pu[cur];
                }
                ret.reverse();
                return Some(ret);
            }
            let nxt = adj[x][d];
            if pu[nxt] == n {
                pu[nxt] = x;
                pd[nxt] = d;
                q.push_back(nxt);
            }
        }
    }
    None
}
fn cmp(a: &[u8], b: &[u8]) -> bool { // returns true if a < b
    if a.len() != b.len() { return a.len() < b.len(); }
    for i in 0..a.len() {
        if a[i] != b[i] { return a[i] < b[i]; }
    }
    false
}
fn get(bit: usize) -> (usize, Vec<usize>) {
    let mut ret = vec![];
    let mut cnt = 0;
    for d in 0..10 {
        if bit & (1 << d) != 0 {
            ret.push(d);
            cnt += 1;
        }
    }
    (cnt, ret)
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    if n < 10 { writeln!(so, "{}", n).unwrap(); return; }
    let mut adj = vec![vec![0; 10]; n];
    for i in 0..n {
        for j in 0..10 {
            adj[i][j] = (i * 10 + j) % n;
        }
    }

    let (mut ac, mut av) = (10, vec![]);
    for bit in 1..1<<10 {
        let (c, digits) = get(bit);
        if c > ac { continue; }
        if let Some(v) = bfs(&adj, &digits, n) {
            if c < ac {
                ac = c;
                av = v;
            } else if c == ac && cmp(&v, &av) {
                av = v;
            }
        }
    }
    for &d in &av {
        write!(so, "{}", d).unwrap();
    }
    writeln!(so).unwrap();
}