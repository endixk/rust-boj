// BOJ 18593 [TDL]
fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
fn f(n: u64, m: u64) -> u64 {
    let mut c = 0;
    for x in n+1.. {
        if gcd(x, n) == 1 { c += 1; }
        if c == m { return x; }
    }
    unreachable!();
}
pub fn main() { read();
    for _ in 0..next() {
        let (k, m) = (next::<u64>(), next::<u64>());
        let mut ans = u64::MAX;
        for x in 1..666 {
            let n = k ^ x;
            if n < 2 { continue; }
            if f(n, m) == n + x { ans = ans.min(n); }
        }
        println!("{}", if ans == u64::MAX { "-1".to_string() } else { ans.to_string() });
    }
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