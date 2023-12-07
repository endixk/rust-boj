// BOJ 30831 [Pre/Post-Requisite Subjects]
// Supported by GitHub Copilot

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x { x }
    else {
        root[x] = find(root, root[x]);
        root[x]
    }
}
fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, x: usize, y: usize) {
    let (x, y) = (find(root, x), find(root, y));
    if x == y { return; }
    if rank[x] < rank[y] {
        root[x] = y;
    } else {
        root[y] = x;
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
    }
}

pub fn main() { read();
    let (n, a, b) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut nxt = vec![0; n+1];
    for _ in 0..a {
        let (u, v) = (next::<usize>(), next::<usize>());
        nxt[u] = v;
    }

    let mut root = (0..=n).collect::<Vec<_>>();
    let mut rank = vec![0; n+1];
    let mut get = (0..=n).collect::<Vec<_>>();
    let mut fnx = vec![0; n+1];
    for _ in 0..b {
        let (u, v) = (next::<usize>(), next::<usize>());
        let p = get[find(&mut root, u)];
        union(&mut root, &mut rank, u, v);
        fnx[u] = v;
        get[find(&mut root, u)] = p;
    }

    let mut vis = vec![false; n+1];
    for _ in 0..next() {
        // for i in 1..=n { print!("{} ", get[find(&mut root, i)]); } println!(); // debug
        let x = find(&mut root, next::<usize>());
        let u = get[x];
        if vis[u] { println!("-1"); continue; }
        else { println!("{}", u); vis[u] = true; }
        if fnx[u] > 0 {
            get[x] = fnx[u];
        } else if nxt[u] > 0 {
            let v = nxt[u];
            let w = get[find(&mut root, v)];
            assert_ne!(u, w);
            union(&mut root, &mut rank, u, v);
            let u = find(&mut root, u);
            get[u] = w;
        }
    }
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