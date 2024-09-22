// BOJ 18561 [Everything Has Changed]
use std::f64::consts::PI;
fn dist(ax: f64, ay: f64, bx: f64, by: f64) -> f64 { (ax - bx).hypot(ay - by) }
fn angle(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) -> f64 {
    let (a, b, c) = (dist(bx, by, cx, cy), dist(ax, ay, cx, cy), dist(ax, ay, bx, by));
    ((a.powi(2) + c.powi(2) - b.powi(2)) / (2.0 * a * c)).acos()
}
pub fn main() {
    let p = &mut input(277777);
    for _ in 0..i16(p) {
        let (n, r1) = (i16(p), i16(p) as i64);
        let mut ans = 2.0 * PI * r1 as f64;
        for _ in 0..n {
            let (a, b, r2) = (i16(p) as i64, i16(p) as i64, i16(p) as i64);
            let (asq, bsq, r1sq, r2sq) = (a*a, b*b, r1*r1, r2*r2);
            let dsq = asq + bsq;
            if dsq == (r1 - r2).pow(2) {
                if r1 > r2 { ans += 2.0 * PI * r2 as f64; }
                continue;
            }
            if dsq == (r1 + r2).pow(2) { continue; }
            if dsq > (r1 + r2).pow(2) || dsq < (r1 - r2).pow(2) { continue; }

            let c = r1sq - r2sq + dsq;
            let d = asq * c * c - dsq * (c * c - 4 * bsq * r1sq);
            let x1 = ((a * c) as f64 - (d as f64).sqrt()) / (2 * dsq) as f64;
            let y1 = if b == 0 { (r1sq as f64 - x1 * x1).sqrt() }
            else { (c as f64 - 2.0 * a as f64 * x1) / (2.0 * b as f64) };
            ans -= angle(x1, y1, 0.0, 0.0, a as f64, b as f64) * 2.0 * r1 as f64;
            ans += angle(x1, y1, a as f64, b as f64, 0.0, 0.0) * 2.0 * r2 as f64;
        }
        println!("{:.10}", ans);
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
thread_local! {
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
extern "C" { fn mmap(a: *mut u8, l: usize, p: i32, f: i32, d: i32, o: i64) -> *mut u8; }
fn input(size: usize) -> *const u8 { unsafe { mmap(0 as *mut u8, size, 1, 2, 0, 0) } }
fn i16(p: &mut *const u8) -> i16 { unsafe {
    let mut n = 0;
    let neg = if **p == b'-' { *p = p.offset(1); true } else { false };
    while **p & 16 != 0 { n = n * 10 + (**p as i16 & 15); *p = p.offset(1) }
    *p = p.offset(1);
    if neg { -n } else { n }
}}