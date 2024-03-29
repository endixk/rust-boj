// BOJ 14464 [Why Did the Cow Cross the Road (Silver)]
pub fn main() { read();
    let (c, n) = (next::<usize>(), next::<usize>());
    let mut t = (0..c).map(|_| next::<i32>()).collect::<Vec<_>>();
    t.sort_unstable();
    let mut v = (0..n).map(|_| (next::<i32>(), next::<i32>())).collect::<Vec<_>>();
    v.sort_unstable();

    let (mut i, mut c) = (0, 0);
    let mut pq = std::collections::BinaryHeap::new();
    for x in t {
        while i < n && v[i].0 <= x {
            pq.push(-v[i].1);
            i += 1;
        }
        while let Some(y) = pq.pop() {
            if x <= -y {
                c += 1;
                break;
            }
        }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}