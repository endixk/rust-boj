// BOJ 28976 [Circular Roads]
// Supported by GitHub Copilot

#[inline] fn deg(a: i64, b: i64) -> f64 {
    let c = (a - b).abs() % 360_000_000;
    ((if c > 180_000_000 { 360_000_000 - c } else { c }) as f64 * 1e-6).to_radians()
}
pub fn main() { read();
    let (n, q) = (next::<usize>(), next::<usize>());
    let mut ang = vec![];
    for _ in 0..n {
        let a = (next::<f64>() * 1e6 + 1e-6) as i64;
        ang.push(a);
        ang.push(a - 360_000_000);
        ang.push(a + 360_000_000);
    }
    ang.sort_unstable();

    for _ in 0..q {
        let a = (next::<f64>() * 1e6 + 1e-6) as i64;
        let b = (next::<f64>() * 1e6 + 1e-6) as i64;
        if ang.binary_search(&b).is_ok() {
            println!("{:.10}", 10.0 + 10.0 * deg(a, b));
            continue;
        }

        let p = ang.partition_point(|&x| x < b);
        let a1 = 10.0 + 10.0 * deg(a, ang[p-1]) + 20.0 * deg(b, ang[p-1]);
        let a2 = 10.0 + 10.0 * deg(a, ang[p]) + 20.0 * deg(b, ang[p]);
        println!("{:.10}", a1.min(a2));
    }
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