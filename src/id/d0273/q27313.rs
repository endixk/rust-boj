// BOJ 27313 [Watching Animes Efficiently]
pub fn main() { read();
    let (n, m, k) = (next::<usize>(), next::<i64>(), next::<usize>());
    let mut v = (0..n).map(|_| next::<i64>()).collect::<Vec<_>>();
    v.sort_unstable();

    let mut c = 0;
    let mut pq = std::collections::BinaryHeap::new();
    for i in 0..k.min(n) {
        if v[i] > m { println!("{}", c); return; }
        pq.push(-v[i]); c += 1;
    }
    for i in k..n {
        let x = -pq.pop().unwrap();
        if x + v[i] > m { println!("{}", c); return; }
        pq.push(-x-v[i]); c += 1;
    }
    println!("{}", c);
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