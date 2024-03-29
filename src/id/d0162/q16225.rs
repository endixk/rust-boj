// BOJ 16225 [The 271st Well-known Cup]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut v = vec![(0, 0); n];
    for i in 0..n { v[i].0 = next::<usize>(); }
    for i in 0..n { v[i].1 = next::<usize>(); }
    v.sort_unstable_by(|x, y| x.1.cmp(&y.1).then(y.0.cmp(&x.0)));

    let mut ans = v[0].0;
    let mut j = 1;
    let mut pq = std::collections::BinaryHeap::new();
    for _ in 1..n/2 {
        pq.push(v[j].0);
        pq.push(v[j+1].0);
        j += 2;
        ans += pq.pop().unwrap();
    }
    println!("{}", ans);
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