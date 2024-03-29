// BOJ 23752 [Riffle Shuffle]
// Supported by GitHub Copilot

fn pmod(mut x: usize, mut p: usize, m: usize) -> usize {
    let mut ret = 1;
    while p > 0 {
        if p & 1 == 1 { ret = ret * x % m; }
        x = x * x % m;
        p >>= 1;
    }
    ret
}
pub fn main() { read();
    let (n, q, mut p) = (next::<usize>(), next::<usize>(), next::<usize>()-1);
    for _ in 0..q {
        let q = next::<u8>();
        if q == 1 {
            let (x, y, z) = (next::<usize>()-1, next::<usize>()-1, next::<usize>());
            if p < x || y < p { continue; }
            if p == y && (y - x) % 2 == 1 { continue; }
            let (k, c) = (p - x, y - x + if (y - x) % 2 == 0 { 1 } else { 0 });
            p = x + pmod(2, z, c) * k % c;
        } else {
            let x = next::<usize>();
            if p < x { p += n-x; }
            else { p -= x; }
        }
    }
    println!("{}", p+1);
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