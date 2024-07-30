// BOJ 6382 [N-Credible Mazes]
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
#[inline] fn conv(a: &[usize], n: usize) -> usize {
    let mut r = 0;
    for i in 0..n { r = r * 10 + a[i]; }
    r
}
pub fn main() { read();
    for tc in 1.. {
        let n = next::<usize>();
        if n == 0 { break; }
        let mut map = std::collections::HashMap::new();
        let mut it = 0;

        let s = conv(&(0..n).map(|_| next()).collect::<Vec<_>>(), n);
        let s = *map.entry(s).or_insert_with(|| { it += 1; it - 1 });
        let e = conv(&(0..n).map(|_| next()).collect::<Vec<_>>(), n);
        let e = *map.entry(e).or_insert_with(|| { it += 1; it - 1 });

        let mut edges = vec![];
        loop {
            let k = next::<i32>();
            if k < 0 { break; }
            let mut v = vec![k as usize];
            for _ in 1..n { v.push(next()); }
            let v = conv(&v, n);
            let v = *map.entry(v).or_insert_with(|| { it += 1; it - 1 });
            let w = conv(&(0..n).map(|_| next()).collect::<Vec<_>>(), n);
            let w = *map.entry(w).or_insert_with(|| { it += 1; it - 1 });
            edges.push((v, w));
        }

        let mut root = (0..it).collect::<Vec<_>>();
        let mut rank = vec![0; it];
        for (x, y) in edges {
            union(&mut root, &mut rank, x, y);
        }
        println!("Maze #{} {} be travelled", tc, if find(&mut root, s) == find(&mut root, e) { "can" } else { "cannot" });
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}