// BOJ 30848 [Colonization]
// Supported by GitHub Copilot

#[inline] fn tri_area(a: f64, b: f64, c: f64) -> f64 {
    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}
#[inline] fn angle(a: f64, b: f64, c: f64) -> f64 {
    let ret = ((b.powi(2) + c.powi(2) - a.powi(2)) / (2.0 * b * c)).acos();
    ret
}
use std::f64::consts::PI;
fn area(a: f64, b: f64, c: f64) -> f64 {
    let mut ret = 0.0;
    ret += tri_area(a+b, b+c, c+a); // triangle
    ret += a * a * (PI - angle(b+c, c+a, a+b) * 0.5); // circle a
    ret += b * b * (PI - angle(c+a, a+b, b+c) * 0.5); // circle b
    ret += c * c * (PI - angle(a+b, b+c, c+a) * 0.5); // circle c
    ret

}
pub fn main() { read();
    let n = next::<usize>();
    if n > 4 { println!("-1"); return; }

    let mut a = (0..n).map(|_| next::<i32>()).collect::<Vec<_>>();
    if n < 3 {
        println!("{}", a.iter().map(|&i| (i as f64).powi(2) * PI).sum::<f64>());
        return;
    }
    a.sort_unstable_by(|a, b| b.cmp(a));
    let (x, y, z) = (a[0] as f64, a[1] as f64, a[2] as f64);
    let r = 1.0 / (1.0 / x + 1.0 / y + 1.0 / z + 2.0 * (1.0 / x / y + 1.0 / y / z + 1.0 / z / x).sqrt());
    if n == 4 {
        if (a[3] as f64 - r).abs() > 1e-9 { println!("-1"); return; }
    }
    println!("{:.10}", area(x, y, z));

    if n == 3 {
        unsafe {
            if IT.as_mut().unwrap().peek().is_some() {
                println!("{:.10}", r);
            }
        }
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<std::iter::Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}