// BOJ 30964 [flippy mex]
// Supported by GitHub Copilot

fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x { x }
    else {
        root[x] = find(root, root[x]);
        root[x]
    }
}
fn union(root: &mut Vec<usize>, rank: &mut Vec<usize>, max: &mut Vec<usize>, cyc: &mut Vec<bool>, x: usize, y: usize) -> usize {
    let (x, y) = (find(root, x), find(root, y));
    if x == y { return x; }
    return if rank[x] < rank[y] {
        root[x] = y;
        max[y] = max[y].max(max[x]);
        cyc[y] |= cyc[x];
        y
    } else {
        root[y] = x;
        max[x] = max[x].max(max[y]);
        cyc[x] |= cyc[y];
        if rank[x] == rank[y] {
            rank[x] += 1;
        }
        x
    }
}
const MAX: usize = 1_000_002;
pub fn main() { read();
    let n = next::<usize>();
    let mut mex = vec![true; MAX];
    let mut inc = vec![false; MAX];

    let mut root = (0..MAX).collect::<Vec<_>>();
    let mut rank = vec![0; MAX];
    let mut max = (0..MAX).collect::<Vec<_>>();
    let mut cyc = vec![false; MAX];
    let (mut x, mut y) = (0, 0);
    for _ in 0..n {
        let (a, b) = (next::<usize>(), next::<usize>());
        if a == b {
            if mex[a] && x >= a { y += 1; }
            mex[a] = false;
        }
        let (a, b) = (find(&mut root, a), find(&mut root, b));
        if a == b {
            cyc[a] = true;
            inc[max[a]] = true;
        } else {
            let (p, q) = if max[a] < max[b] { (max[a], max[b]) } else { (max[b], max[a]) };
            inc[p] = true;
            let r = union(&mut root, &mut rank, &mut max, &mut cyc, a, b);
            if cyc[r] { inc[q] = true; }
        }
        while inc[x] {
            x += 1;
            if !mex[x] { y += 1; }
        }
        println!("{}", x + 1 - y);
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