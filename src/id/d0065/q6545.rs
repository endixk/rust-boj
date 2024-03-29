// BOJ 6545 [Drink, on Ice]
pub fn main() { read();
    'x: loop {
        let (mw, mi, tw, ti) = (next::<f64>(), next::<f64>(), next::<f64>(), next::<f64>());
        if mw == 0.0 && mi == 0.0 && tw == 0.0 && ti == 0.0 { break; }
        let mut j = 2.09 * mi * (ti + 30.0) + 2.09 * mw * 30.0 + 335.0 * mw + 4.19 * mw * tw;
        let m = mw + mi;

        if j < m * 2.09 * 30.0 {
            println!("{:.1} g of ice and 0.0 g of water at {:.1} C", m, -30.0 + j / (2.09 * m));
            continue 'x;
        }
        j -= m * 2.09 * 30.0;
        if j < m * 335.0 {
            println!("{:.1} g of ice and {:.1} g of water at 0.0 C", m - j / 335.0, j / 335.0);
            continue 'x;
        }
        j -= m * 335.0;
        println!("0.0 g of ice and {:.1} g of water at {:.1} C", m, j / (4.19 * m));
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