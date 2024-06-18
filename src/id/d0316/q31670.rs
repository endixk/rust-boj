// BOJ 31670 [Exceptional Magic Attack]
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<u32>()).collect::<Vec<_>>();
    if n == 1 { println!("0"); return; }

    let (mut x, mut y) = (a[0] as u64, a[1] as u64);
    for i in 2..n {
        if i & 1 == 0 { x = x.min(y) + a[i] as u64; }
        else { y = x.min(y) + a[i] as u64; }
    }
    println!("{}", x.min(y));
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