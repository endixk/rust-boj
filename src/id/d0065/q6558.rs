// BOJ 6558 [Average is not Fast Enough!]
pub fn main() { read();
    let (n, d) = (next::<usize>(), next::<f64>());
    while peek() {
        let t = next::<i32>();
        let (mut x, mut f) = (0, true);
        for _ in 0..n {
            let mut k = 3600;
            for w in next::<String>().split(":") {
                if w.chars().next().unwrap() == '-' { f = false }
                else {
                    x += w.parse::<u32>().unwrap() * k;
                    k /= 60;
                }
            }
        }
        if f {
            let x = (x as f64 / d).round() as i32;
            println!("{:>3}: {}:{:0>2} min/km", t, x/60, x%60);
        }
        else {
            println!("{:>3}: -", t);
        }
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
