// BOJ 6595 [Frogger]
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
    for tc in 1.. {
        let n = next::<usize>();
        if n == 0 { break; }
        println!("Scenario #{}", tc);

        let c = (0..n).map(|_| (next::<i32>(), next::<i32>())).collect::<Vec<_>>();
        let mut edges = vec![];
        for i in 0..n-1 { for j in i+1..n {
            edges.push(((c[i].0 - c[j].0).pow(2) + (c[i].1 - c[j].1).pow(2), i, j));
        }}
        edges.sort_unstable();

        let mut root = (0..n).collect::<Vec<_>>();
        let mut rank = vec![0; n];
        for (w, u, v) in edges {
            union(&mut root, &mut rank, u, v);
            if find(&mut root, 0) == find(&mut root, 1) {
                println!("Frog Distance = {:.3}", (w as f64).sqrt());
                break;
            }
        }
        println!();
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