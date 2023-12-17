// BOJ 28962 [Faceted Glasses]
// Supported by GitHub Copilot

#[derive(Eq, PartialEq, Clone, Copy, Debug)] struct Point { x: i32, y: i32 }
fn area(poly: &Vec<Point>) -> f64 {
    let mut ret = 0.0;
    for i in 0..poly.len() {
        let j = (i + 1) % poly.len();
        ret += poly[i].x as f64 * poly[j].y as f64 - poly[i].y as f64 * poly[j].x as f64;
    }
    ret.abs() / 2.0
}
pub fn main() { read();
    let (n, v) = (next::<usize>(), next::<f64>());
    let mut a = 0.0;
    for _ in 0..n {
        let k = next::<usize>();
        let poly = (0..k).map(|_| Point { x: next(), y: next() }).collect::<Vec<_>>();
        a += area(&poly);
    }
    println!("{:.10}", v / a);
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