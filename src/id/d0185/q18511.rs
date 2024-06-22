// BOJ 18511 [Constructing the Largest Number]
fn gen(x: u32, n: u32, v: &[u32], ans: &mut u32) {
    if x > n { return; }
    if x > *ans { *ans = x; }
    for &i in v.iter() {
        gen(x * 10 + i, n, v, ans);
    }
}
pub fn main() { read();
    let (n, k) = (next::<u32>(), next::<usize>());
    let v = (0..k).map(|_| next::<u32>()).collect::<Vec<_>>();
    let mut ans = 0;
    gen(0, n, &v, &mut ans);
    println!("{}", ans);
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