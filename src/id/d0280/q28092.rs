// BOJ 28092 [Priority and Queries]
fn find(root: &mut Vec<usize>, x: usize) -> usize {
    if root[x] == x { x }
    else {
        root[x] = find(root, root[x]);
        root[x]
    }
}
fn union(root: &mut Vec<usize>, size: &mut Vec<usize>, x: usize, y: usize) {
    if x > y {
        root[x] = y;
        size[y] += size[x];
    } else {
        root[y] = x;
        size[x] += size[y];
    }
}
pub fn main() { read();
    let (n, q) = (next::<usize>(), next::<usize>());
    let mut tag = vec![true; n];
    let mut root = (0..n).collect::<Vec<usize>>();
    let mut size = vec![1; n];

    let mut pq = std::collections::BinaryHeap::new();
    for i in 0..n { pq.push((1, n-i)); }
    for _ in 0..q {
        let x = next::<u8>();
        if x == 1 {
            let (u, v) = (next::<usize>()-1, next::<usize>()-1);
            let (u, v) = (find(&mut root, u), find(&mut root, v));
            if u == v { tag[u] = false; }
            else {
                union(&mut root, &mut size, u, v);
                let (i, j) = if u < v { (u, v) } else { (v, u) };
                if tag[j] {
                    tag[j] = false;
                    pq.push((size[i], n-i));
                } else { tag[i] = false; }
            }
        } else {
            loop {
                let (_, i) = pq.pop().unwrap();
                let i = n-i;
                if tag[i] {
                    println!("{}", i+1);
                    tag[i] = false;
                    break;
                }
            }
        }
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}