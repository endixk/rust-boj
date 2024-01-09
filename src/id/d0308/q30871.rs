// BOJ 30871 [The Brilliance of ChatGPT]
// Supported by GitHub Copilot

fn f(x: u64, l: &[u64], r: &[u64]) -> bool {
    let mut val = x;
    for (&l, &r) in l.iter().zip(r.iter()) {
        if l <= x && x <= r {
            val = val^((x|l).wrapping_add((x&r).wrapping_mul(l^r)));
        }
    }
    val >= 0x0123456789ABCDEF
}
pub fn main() { read();
    let n = next::<usize>();
    let l = (0..n).map(|_| next::<u64>()).collect::<Vec<_>>();
    let r = (0..n).map(|_| next::<u64>()).collect::<Vec<_>>();

    let (mut s, mut e) = (0, 1e18 as u64 + 1);
    while s+1 < e {
        let m = s+e>>1;
        if f(m, &l, &r) { e = m; } else { s = m; }
    }
    println!("{}", s);
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