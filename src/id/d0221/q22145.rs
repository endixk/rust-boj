// BOJ 22145 [Coronation]
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn ptr(p: &mut *const u8) -> usize { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as usize & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
fn dfs(adj: &Vec<Vec<(usize, usize)>>, u: usize, p: usize, d: usize, par: &mut Vec<(usize, usize)>) {
    par[u] = (p, d);
    for &(v, d) in &adj[u] {
        if v == p { continue; }
        dfs(adj, v, u, d, par);
    }
}
pub fn main() {
    let mut p = input(2000000);
    let (n, x, y) = (ptr(&mut p), ptr(&mut p), ptr(&mut p));
    let mut adj = vec![vec![]; n+1];
    for _ in 1..n {
        let (u, v, d) = (ptr(&mut p), ptr(&mut p), ptr(&mut p));
        adj[u].push((v, d)); adj[v].push((u, d));
    }

    let mut par = vec![(0, 0); n+1];
    dfs(&adj, x, 0, 0, &mut par);

    let mut path = vec![];
    let (mut u, mut d) = (y, 9999);
    while u != x {
        path.push(u);
        (u, d) = (par[u].0, d.min(par[u].1))
    }
    path.push(x);

    let k = adj[x].iter().position(|&(v, _)| v == path[path.len() - 2]).unwrap();
    adj[x][k].1 -= d;
    let k = adj[y].iter().position(|&(v, _)| v == path[1]).unwrap();
    adj[y][k].1 -= d;

    println!("{}", d + adj[x].iter().map(|&(_, d)| d).max().unwrap().min(adj[y].iter().map(|&(_, d)| d).max().unwrap()));
}