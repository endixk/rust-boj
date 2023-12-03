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

fn mst(e: &[(usize, usize, usize, usize)], n: usize) -> (usize, bool) {
    let mut e = e.to_vec();
    e.sort_unstable();
    let mut root = (0..=n).collect::<Vec<_>>();
    let mut rank = vec![0; n+1];
    let mut mst = 0;
    for &(c, _, u, v) in e.iter() {
        let (u, v) = (find(&mut root, u), find(&mut root, v));
        if u == v { continue; }
        union(&mut root, &mut rank, u, v);
        mst += c;
    }
    let r = find(&mut root, 1);
    (mst, (2..=n).all(|i| find(&mut root, i) == r))
}
pub fn main() { read();
    let (n, q) = (next::<usize>(), next::<usize>());
    let mut e = vec![];
    for _ in 0..q {
        let (u, v, c, t) = (next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>());
        e.push((c, t, u, v));
    }

    let (m, x) = mst(&e, n);
    if !x { println!("-1"); return; }

    let mut v = e.iter().map(|&(_, t, _, _)| t).collect::<Vec<_>>();
    v.sort_unstable(); v.dedup();
    let (mut l, mut r) = (0, v.len());
    while l < r {
        let mid = (l + r) / 2;
        let e = e.iter().filter(|&&(_, t, _, _)| t <= v[mid]).map(|&x| x).collect::<Vec<_>>();
        let (k, x) = mst(&e, n);
        if x && k == m { r = mid; }
        else { l = mid + 1; }
    }

    println!("{} {}", v[l], m);
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