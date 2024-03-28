// BOJ 31563 [Sequence Rotation and Queries]
// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = usize; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
pub fn main() {
    let mut p = input(6666666);
    let (n, q) = (ptr(&mut p), ptr(&mut p));
    let mut x = vec![0; n+1];
    for i in 1..=n { x[i] = x[i-1] + ptr(&mut p) }
    let mut r = 0;

    for _ in 0..q {
        match ptr(&mut p) {
            1 => {
                let k = ptr(&mut p);
                r = (r + n - k) % n;
            },
            2 => {
                let k = ptr(&mut p);
                r = (r + k) % n;
            },
            _ => {
                let (a, b) = (ptr(&mut p) - 1, ptr(&mut p) - 1);
                let (a, b) = ((a + r) % n + 1, (b + r) % n + 1);
                if a > b { println!("{}", x[n] - x[a-1] + x[b]); }
                else { println!("{}", x[b] - x[a-1]); }
            }
        }
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
use println;
use std::{io::*, cell::*, str::*};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}