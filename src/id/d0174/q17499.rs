// BOJ 17499 [Sequence and Shift Queries]
pub fn main() { read();
    let (n, q) = (next::<usize>(), next::<usize>());
    let mut a = (0..n).map(|_| next::<i32>()).collect::<Vec<_>>();
    let mut p = 0;
    for _ in 0..q {
        let t = next::<u8>();
        if t == 1 {
            let (i, x) = (next::<usize>(), next::<i32>());
            a[(i + p - 1) % n] += x;
        } else {
            let i = next::<usize>();
            if t == 2 { p = (p + n - i) % n; }
            else { p = (p + i) % n; }
        }
    }
    for i in 0..n { print!("{} ", a[(i + p) % n]); }
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