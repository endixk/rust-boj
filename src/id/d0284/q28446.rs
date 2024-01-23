// BOJ 28446 [Bowling Balls]

// fast I/O snippet from: https://gist.github.com/kiwiyou/bea8be80e35211fbedc5b780c22ebfe9
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
type I = u32; fn ptr(p: &mut *const u8) -> I { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as I & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}

use std::collections::HashMap;
pub fn main() {
    let mut p = input(1 << 22);
    let mut map = HashMap::new();
    for _ in 0..ptr(&mut p) {
        match ptr(&mut p) {
            1 => {
                let (a, b) = (ptr(&mut p), ptr(&mut p));
                map.insert(b, a);
            },
            _ => println!("{}", map.get(&ptr(&mut p)).unwrap()),
        }
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
use println;
use std::{io::*, cell::*, str::*, fmt::Debug};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}