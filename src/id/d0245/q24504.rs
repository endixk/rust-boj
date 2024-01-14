// BOJ 24504 [blobcry]
fn dfs(u: usize, p: usize, adj: &Vec<Vec<(usize, bool)>>,
       dfsn: &mut Vec<usize>, cnt: &mut usize, arts: &mut Vec<(usize, usize)>) -> usize {
    *cnt += 1;
    dfsn[u] = *cnt;
    let mut ret = dfsn[u];
    for &(v, _) in adj[u].iter() {
        if v == p { continue; }
        if dfsn[v] == 0 {
            let prev = dfs(v, u, adj, dfsn, cnt, arts);
            if prev > dfsn[u] {
                if u < v { arts.push((u, v)); }
                else { arts.push((v, u)); }
            }
            ret = ret.min(prev);
        } else {
            ret = ret.min(dfsn[v]);
        }
    }
    ret
}
fn connect(u: usize, c: usize, cid: &mut Vec<usize>, adj: &Vec<Vec<(usize, bool)>>) -> usize {
    cid[u] = c;
    let mut ret = adj[u].iter().filter(|&&(v, a)| a && cid[v] == 0).count();
    for &(v, a) in adj[u].iter() {
        if !a { continue; }
        if cid[v] == 0 {
            ret += connect(v, c, cid, adj);
        }
    }
    ret
}
fn tree_dfs(u: usize, p: usize, tree: &Vec<Vec<usize>>, csz: &Vec<usize>, sub: &mut Vec<usize>) {
    sub[u] = csz[u];
    for &v in tree[u].iter() {
        if v == p { continue; }
        tree_dfs(v, u, tree, csz, sub);
        sub[u] += sub[v] + 1;
    }
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    if m % 2 == 0 { println!("0"); return; }

    let mut edges = vec![];
    let mut adj = vec![vec![]; n+1];
    for _ in 0..m {
        let (u, v) = (next::<usize>(), next::<usize>());
        edges.push((u, v));
        adj[u].push((v, true)); adj[v].push((u, true));
    }
    for u in 1..=n { adj[u].sort_unstable(); }

    // find articulation bridges
    let mut dfsn = vec![0; n+1];
    let mut cnt = 0;
    let mut arts = vec![];
    for u in 1..=n {
        if dfsn[u] == 0 {
            dfs(u, n, &adj, &mut dfsn, &mut cnt, &mut arts);
        }
    }

    // color articulation bridges
    for &(u, v) in &arts {
        let k = adj[u].binary_search(&(v, true)).unwrap();
        adj[u][k].1 = false;
        let k = adj[v].binary_search(&(u, true)).unwrap();
        adj[v][k].1 = false;
    }

    // coalesce components
    let mut cid = vec![0; n+1];
    let mut csz = vec![0];
    let mut cit = 1;
    for i in 1..=n {
        if cid[i] == 0 {
            csz.push(connect(i, cit, &mut cid, &adj));
            cit += 1;
        }
    }

    // generate a tree out of articulation bridges
    let mut tree = vec![vec![]; cit];
    for &(u, v) in &arts {
        let (cu, cv) = (cid[u], cid[v]);
        tree[cu].push(cv); tree[cv].push(cu);
    }
    let mut sub = vec![0; cit];
    tree_dfs(1, 0, &tree, &csz, &mut sub);

    // count articulation bridges that splits the tree with even components
    for (u, v) in arts {
        let (cu, cv) = (cid[u], cid[v]);
        let s = if sub[cu] < sub[cv] { sub[cu] } else { sub[cv] };
        if s % 2 == 0 {
            let k = adj[u].binary_search(&(v, false)).unwrap();
            adj[u][k].1 = true;
            let k = adj[v].binary_search(&(u, false)).unwrap();
            adj[v][k].1 = true;
        }
    }

    // collect results
    let mut r = vec![];
    for (i, (u, v)) in edges.into_iter().enumerate() {
        let k = if let Ok(k) = adj[u].binary_search(&(v, false)) { k }
                else { adj[u].binary_search(&(v, true)).unwrap() };
        if adj[u][k].1 { r.push(i+1); }
    }
    println!("{}", r.len());
    for i in r { print!("{} ", i); }
    println!();
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}