// BOJ 15773 [Touch The Sky]
pub fn main() { read();
    let n = next::<usize>();
    let mut a = (0..n).map(|_| (next::<u64>(), next::<u64>())).collect::<Vec<_>>();
    a.sort_unstable_by_key(|x| x.0 + x.1);

    let mut pq = std::collections::BinaryHeap::new();
    let mut h = 0;
    for (l, d) in a {
        if l >= h { pq.push(d); h += d; }
        else {
            let k = pq.pop().unwrap(); h -= k;
            pq.push(k.min(d)); h += k.min(d);
        }
    }

    println!("{}", pq.len());
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