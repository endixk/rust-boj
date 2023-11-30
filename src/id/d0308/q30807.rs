// BOJ 30807 [TSM]
// Supported by GitHub Copilot

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
fn kruskal(edges: &Vec<(usize,usize,usize)>, n: usize) -> usize {
    let mut root = (0..n).collect::<Vec<_>>();
    let mut rank = vec![0; n];
    let mut ret = 0;
    for (w, u, v) in edges {
        if find(&mut root, *u) != find(&mut root, *v) {
            union(&mut root, &mut rank, *u, *v);
            ret += w;
        }
    }
    ret
}
fn go(e: &Vec<(usize,usize,usize,usize)>, n: usize, k: usize) -> usize {
    let mut edges = vec![];
    let mut k = k;
    for &(u, v, x, y) in e.iter() {
        if k == 0 {
            edges.push((x, u, v));
        } else if k > y - x {
            edges.push((y, u, v));
            k -= y - x;
        } else {
            edges.push((x + k, u, v));
            k = 0;
        }
    }
    edges.sort_unstable();
    return kruskal(&edges, n);
}

pub fn main() {
    let s = read();
    let mut it = s.split_ascii_whitespace();

    let (n, m, k) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut e = vec![];
    let mut s = 0;
    for _ in 0..m {
        let (u, v) = (next::<usize>(&mut it) - 1, next::<usize>(&mut it) - 1);
        let (x, y) = (next::<usize>(&mut it), next::<usize>(&mut it));
        e.push((u, v, x, y));
        s += y - x;
    }

    let (mut lo, mut hi) = (0, s);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if go(&e, n, mid) >= k {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    if go(&e, n, lo) == k {
        println!("YES");
    } else {
        println!("NO");
        return;
    }

    for (_, _, x, y) in e {
        if lo == 0 {
            println!("{}", x);
        } else if lo > y - x {
            println!("{}", y);
            lo -= y - x;
        } else {
            println!("{}", x + lo);
            lo = 0;
        }
    }
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