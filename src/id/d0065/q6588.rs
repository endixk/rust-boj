// BOJ 6588 [Goldbach's Conjecture]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = u32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}

fn sieve(n: usize) -> (Vec<bool>, Vec<usize>) {
    let mut isp = vec![true; n + 1];
    let mut p = vec![];
    isp[0] = false; isp[1] = false; isp[2] = false;
    for i in (3..=n).step_by(2) {
        if isp[i] {
            p.push(i);
            for j in (i * i..=n).step_by(i) {
                isp[j] = false;
            }
        }
    }
    (isp, p)
}
pub fn main() {
    let (isp, p) = sieve(1000000);
    let mut buf = input(1000000);
    loop {
        let n = ptr(&mut buf) as usize;
        if n == 0 { break; }
        for &p in p.iter() {
            if isp[n - p] {
                println!("{} = {} + {}", n, p, n - p);
                break;
            }
        }
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}