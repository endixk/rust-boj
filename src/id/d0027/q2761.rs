// BOJ 2761 [Model Rocket Height]
pub fn main() { read();
    let (d, h) = (next::<f64>(), next::<f64>());
    loop {
        let (a, b, c) = (next::<f64>(), next::<f64>(), next::<f64>());
        if a == 0.0 { break; }

        let (xa, xb, xc) = (
            a.to_radians().tan().powi(2).recip(),
            b.to_radians().tan().powi(2).recip(),
            c.to_radians().tan().powi(2).recip()
        );
        println!("{:.0}", d * (2.0 / (xa + xc - 2.0 * xb)).sqrt() + h);
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