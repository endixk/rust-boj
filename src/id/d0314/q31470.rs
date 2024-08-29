// BOJ 31470 [L-Knight Game (Bitter)]
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn ptr(p: &mut *const u8) -> u64 { unsafe {
    let mut n = 0;
    while **p & 16 != 0 { n = n * 10 + (**p as u64 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    n
}}
fn count(init: u64, size: u64, step: u64) -> u64 {
    if init >= size { return 0; }
    (size + step - 1 - init) / step
}
fn get(w: u64, h: u64, x: u64, y: u64, i: u64, j: u64) -> u64 {
    let (x1, y1) = (count(i, w, x * 2), count(j, h, y * 2));
    let (x2, y2) = (count(if i < x { i + x } else { i - x }, w, x * 2), count(j + y, h, y * 2));
    (x1 * y1).max(x2 * y2)
}
pub fn main() {
    let mut p = input(4400044);
    for _ in 0..ptr(&mut p) {
        let (w, h, x, y) = (ptr(&mut p), ptr(&mut p), ptr(&mut p), ptr(&mut p));
        let (m, n) = (w % x, h % y);
        println!("{}",
            m * n * get(w, h, x, y, 0, 0) +
            (x - m) * n * get(w, h, x, y, m, 0) +
            m * n * get(w, h, x, y, x, 0) +
            (x - m) * n * get(w, h, x, y, x + m, 0) +
            m * (y - n) * get(w, h, x, y, 0, n) +
            (x - m) * (y - n) * get(w, h, x, y, m, n) +
            m * (y - n) * get(w, h, x, y, x, n) +
            (x - m) * (y - n) * get(w, h, x, y, x + m, n)
        );
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}