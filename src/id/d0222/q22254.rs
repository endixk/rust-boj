// BOJ 22254 [Manufacturing Process Consultant]
use std::collections::BinaryHeap;
use std::cmp::Reverse;
fn solve(t: &[usize], k: usize) -> usize {
    let mut pq = BinaryHeap::new();
    for _ in 0..k { pq.push(Reverse(0)); }
    for &x in t {
        let Reverse(t) = pq.pop().unwrap();
        pq.push(Reverse(t + x));
    }
    pq.into_iter().map(|Reverse(x)| x).max().unwrap()
}
pub fn main() { read();
    let (n, x) = (next::<usize>(), next::<usize>());
    let t = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    let (mut lo, mut hi) = (1, n);
    while lo < hi {
        let mid = (lo + hi) / 2;
        if solve(&t, mid) <= x { hi = mid; }
        else { lo = mid + 1; }
    }
    println!("{}", lo);
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