// BOJ 6600 [The Circumference of the Circle]
fn det2(mat: (f64, f64, f64, f64)) -> f64 { mat.0 * mat.3 - mat.1 * mat.2 }
fn det3(mat: (f64, f64, f64, f64, f64, f64, f64, f64, f64)) -> f64 {
    mat.0 * (mat.4 * mat.8 - mat.5 * mat.7) - mat.1 * (mat.3 * mat.8 - mat.5 * mat.6) + mat.2 * (mat.3 * mat.7 - mat.4 * mat.6)
}
pub fn main() { read();
    use std::f64::consts::PI;
    while peek() {
        let (x1, y1) = (next::<f64>(), next::<f64>());
        let (x2, y2) = (next::<f64>(), next::<f64>());
        let (x3, y3) = (next::<f64>(), next::<f64>());
        let (p1, p2, p3) = (x1.powi(2) + y1.powi(2), x2.powi(2) + y2.powi(2), x3.powi(2) + y3.powi(2));

        let a = det3((x1, y1, 1.0, x2, y2, 1.0, x3, y3, 1.0));
        let bx = -det3((p1, y1, 1.0, p2, y2, 1.0, p3, y3, 1.0));
        let by = det3((p1, x1, 1.0, p2, x2, 1.0, p3, x3, 1.0));
        let c = -det3((p1, x1, y1, p2, x2, y2, p3, x3, y3));

        println!("{:.2}", (bx.powi(2) + by.powi(2) - 4.0 * a * c).sqrt() * PI / a.abs());
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, iter::Peekable};
static mut BUF: String = String::new();
static mut IT: Option<Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }
