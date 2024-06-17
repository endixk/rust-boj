// BOJ 18114 [Black Friday]
pub fn main() { read();
    let (n, c) = (next::<usize>(), next::<i32>());
    let mut w = (0..n).map(|_| next::<i32>()).collect::<Vec<_>>();
    w.push(0); w.push(0); w.sort_unstable();

    for i in 0..n {
        let (mut l, mut r) = (i+1, n+1);
        while l < r {
            if w[i] + w[l] + w[r] == c { println!("1"); return; }
            if w[i] + w[l] + w[r] < c { l += 1; }
            else { r -= 1; }
        }
    }
    println!("0");
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