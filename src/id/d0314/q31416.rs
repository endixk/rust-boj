// BOJ 31416 [Virtual Validation]
pub fn main() { read();
    for _ in 0..next() {
        let (ta, tb, va, vb) = (next::<i32>(), next::<i32>(), next::<i32>(), next::<i32>());
        let mut pq = std::collections::BinaryHeap::new();
        pq.push(0); pq.push(-tb*vb);
        for _ in 0..va {
            let x = pq.pop().unwrap();
            pq.push(x - ta);
        }
        pq.pop();
        println!("{}", -pq.pop().unwrap());
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