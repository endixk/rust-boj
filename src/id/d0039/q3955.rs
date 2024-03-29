// BOJ 3955 [Candy Distribution]
fn xgcd(a: i64, b: i64) -> (i64, i64, i64) {
    return if b == 0 { (a, 1, 0) }
    else {
        let (g, x, y) = xgcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}
fn mmi(a: i64, m: i64) -> i64 {
    let (_, x, _) = xgcd(a, m);
    (x + m) % m
}
pub fn main() { read();
    for _ in 0..next() {
        let (k, c) = (next(), next());
        if c == 1 {
            if k >= 1_000_000_000 { println!("IMPOSSIBLE"); continue; }
            println!("{}", k + 1); continue;
        }
        if k == 1 { println!("1"); continue; }
        if xgcd(k, c).0 != 1 { println!("IMPOSSIBLE"); continue; }
        let q = mmi(c, k);
        if q > 1_000_000_000 { println!("IMPOSSIBLE"); continue; }
        println!("{}", q);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}