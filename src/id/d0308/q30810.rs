// BOJ 30810 [Painting]
// Supported by GitHub Copilot

fn dfs_conn(v: &mut Vec<bool>, adj: &Vec<Vec<(usize,usize)>>, cur: usize, par: usize) {
    v[cur] = true;
    for &(nxt, _) in adj[cur].iter() {
        if nxt == par { continue; }
        if v[nxt] { continue; }
        dfs_conn(v, adj, nxt, cur);
    }
}
fn con(adj: &Vec<Vec<(usize,usize)>>, n: usize) -> bool {
    let mut v = vec![false; n+1];
    dfs_conn(&mut v, adj, 1, 0);
    v.iter().filter(|&&x| x).count() == n
}

fn dfs_path(path: &mut Vec<usize>, v1: &mut Vec<bool>, v2: &mut Vec<bool>,
            adj: &Vec<Vec<(usize,usize)>>, r: &Vec<usize>, cur: usize) {
    path.push(cur);
    if !v1[cur] {
        v1[cur] = true;
        let mut nxt = 0;
        for &(u, c) in adj[cur].iter() {
            if c == r[cur] { nxt = u; break; }
        }
        if !v2[nxt] {
            dfs_path(path, v1, v2, adj, r, nxt);
            path.push(cur);
        } else {
            path.push(nxt);
            dfs_path(path, v1, v2, adj, r, cur);
        }
    } else {
        v2[cur] = true;
        for &(u, _) in adj[cur].iter() {
            if !v2[u] {
                dfs_path(path, v1, v2, adj, r, u);
                path.push(cur);
            }
        }
    }
}

pub fn main() {
    let s = read();
    let mut it = s.split_ascii_whitespace();

    let (n, m, _) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut r = vec![0; n+1];
    (1..=n).for_each(|i| r[i] = next::<usize>(&mut it));

    let mut adj = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v, c) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
        adj[u].push((v, c));
        adj[v].push((u, c));
    }

    // integrity check
    if !con(&adj, n) { println!("NO"); return; }
    for u in 1..=n {
        let mut f = false;
        for &(_, c) in adj[u].iter() {
            if c == r[u] { f = true; break; }
        }
        if !f { println!("NO"); return; }
    }

    let mut v1 = vec![false; n+1];
    let mut v2 = vec![false; n+1];
    let mut path = vec![];
    dfs_path(&mut path, &mut v1, &mut v2, &adj, &r, 1);

    assert!(path.len() <= 1000000);
    println!("YES");
    println!("{}", path.len());
    path.iter().rev().for_each(|&x| print!("{} ", x));
    println!();
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
fn read() -> String {
    let mut s = String::new();
    SI.with(|c| c.borrow_mut().read_to_string(&mut s).unwrap());
    s
}
fn next<T: FromStr>(it: &mut SplitAsciiWhitespace) -> T where <T as FromStr>::Err: Debug {
    it.next().unwrap().parse().unwrap()
}