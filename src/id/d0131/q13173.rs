// BOJ 13173 [Omega]
const MOD: i64 = 1_000_000_007;
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
fn mpow(a: i64, p: i64, m: i64) -> i64 {
    let (mut a, mut p, mut r) = (a, p, 1);
    while p > 0 {
        if p & 1 == 1 { r = r * a % m }
        a = a * a % m;
        p >>= 1;
    }
    r
}
pub fn main() { read();
    let (p, q, n, k) = (next::<i64>(), next::<i64>(), next::<i64>(), next::<i64>());
    if k == 0 { println!("0"); return; }
    if k == n { println!("1"); return; }
    if q == p { println!("0"); return; }
    if q == 0 { println!("1"); return; }
    if p == q<<1 { println!("{}", k * mmi(n, MOD) % MOD); return; }

    let r = q * mmi(p - q, MOD) % MOD;
    let a = mpow(r, k, MOD) - 1;
    let b = mpow(r, n, MOD) - 1;
    println!("{}", a * mmi(b, MOD) % MOD);
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