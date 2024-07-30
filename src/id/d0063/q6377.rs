// BOJ 6377 [Problem Bee]
#[inline] fn valid(i: i32, j: i32) -> bool { i % 2 == j % 2 }
fn dist(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let dx = (x1 - x2).abs();
    let dy = (y1 - y2).abs();
    dx + if dy > dx { (dy - dx) / 2 } else { 0 }
}
pub fn main() { read();
    loop {
        let (k, p1, q1, p2, q2) = (next::<f64>(), next::<f64>(), next::<f64>(), next::<f64>(), next::<f64>());
        if k == 0.0 && p1 == 0.0 && q1 == 0.0 && p2 == 0.0 && q2 == 0.0 { break; }
        let (w, h) = (k * 1.5, k * 3.0f64.sqrt() / 2.0);

        let (mut x1, mut y1, mut d1) = (0, 0, k * 99.9);
        for i in (p1 / w).floor() as i32 - 2..=(p1 / w).ceil() as i32 + 2 {
            for j in (q1 / h).floor() as i32 - 2..=(q1 / h).ceil() as i32 + 2 {
                if valid(i, j) {
                    let (x, y) = (i as f64 * w, j as f64 * h);
                    if d1 > (p1 - x).hypot(q1 - y) {
                        (x1, y1, d1) = (i, j, (p1 - x).hypot(q1 - y));
                    }
                }
            }
        }

        let (mut x2, mut y2, mut d2) = (0, 0, k * 99.9);
        for i in (p2 / w).floor() as i32 - 2..=(p2 / w).ceil() as i32 + 2 {
            for j in (q2 / h).floor() as i32 - 2..=(q2 / h).ceil() as i32 + 2 {
                if valid(i, j) {
                    let (x, y) = (i as f64 * w, j as f64 * h);
                    if d2 > (p2 - x).hypot(q2 - y) {
                        (x2, y2, d2) = (i, j, (p2 - x).hypot(q2 - y));
                    }
                }
            }
        }

        match dist(x1, y1, x2, y2) {
            0 => println!("{:.3}", (p1 - p2).hypot(q1 - q2)),
            x => println!("{:.3}", x as f64 * h * 2.0 + d1 + d2)
        };
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
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