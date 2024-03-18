// BOJ 31478 [Ms. Pony Wants to Play!]
fn pmod(mut a: u64, mut p: u64, m: u64) -> u64 {
    let mut r = 1;
    while p > 0 {
        if p & 1 == 1 { r = r * a % m; }
        a = a * a % m;
        p >>= 1;
    }
    r
}
pub fn main() { read();
    let (a, b, c) = (next::<u64>(), next::<u64>(), next::<u64>());
    let (k, l) = (next::<u64>(), next::<u64>());

    let x = pmod(a, pmod(b, c, 6), 7);
    let mut t = 1;
    while pmod(b, t, a) != 0 { t += 1; }
    let y = b.pow(t as u32) / a * pmod(b, c - t, 7) % 7;
    println!("{}", if (k + y) % 7 == l { 1 } else { 0 } << 1 | if (k + x) % 7 == l { 1 } else { 0 });
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