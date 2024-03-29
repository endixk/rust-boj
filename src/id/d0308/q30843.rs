// BOJ 30843 [Vasya and Apartments]
// Supported by GitHub Copilot

fn go(n: i64, s: i64, k: i64) -> bool {
    (n - k) * (n - k + 1) / 2 <= s - k
}
pub fn main() { read();
    let (n, s) = (next::<i64>(), next::<i64>());
    if n * (n + 1) / 2 < s { println!("1"); return; }
    if n * (n + 1) / 2 == s { println!("0"); return; }
    let (mut l, mut r) = (1, n);
    while l < r {
        let m = (l + r) / 2;
        if go(n, s, m) { r = m; }
        else { l = m + 1; }
    }
    println!("{}", l);
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