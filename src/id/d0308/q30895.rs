// BOJ 30895 [Spear of Shojin]
// Supported by GitHub Copilot

#[inline] fn eff(x: i64, y: i64, m: i64) -> (i64, i64) {
    ((m + x - 1) / x, (m + x + y - 1) / (x + y))
}
pub fn main() { read();
    let (x, y, k) = (next::<i64>(), next::<i64>(), next::<i64>());
    let (mut m, mut n) = eff(x, y, k);
    let mut ans = k;
    let mut c = k / (x + y) * (x + y) + 1;
    while c <= k { c += x + y; }
    for _ in 0..1010101 {
        let (p, q) = eff(x, y, c);
        if p * n < q * m { m = p; n = q; ans = c; }
        c += x + y;
    }
    println!("{}", ans);
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