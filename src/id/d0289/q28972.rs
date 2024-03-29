// BOJ 28972 [Acromantulas]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut pq = std::collections::BinaryHeap::new();
    pq.push((0x3f3f3f3f, 1, 0));

    let mut v = (0..n).map(|i| (next::<usize>(), next::<usize>(), i+1)).collect::<Vec<_>>();
    v.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut ans = vec![0; n];
    for (a, c, i) in v {
        if pq.is_empty() { println!("NO"); return; }
        let (b, d, j) = pq.pop().unwrap();
        if a == b { println!("NO"); return; }
        ans[i-1] = j;
        if c > 0 { pq.push((a, c, i)); }
        if d > 1 { pq.push((b, d - 1, j)); }
    }

    println!("YES");
    for i in ans { print!("{} ", i); }
    println!();
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