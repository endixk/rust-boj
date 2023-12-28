pub fn main() { read();
    for _ in 0..next() {
        let (x1, y1, x2, y2, w) = (next::<i64>(), next::<i64>(), next::<i64>(), next::<i64>(), next::<i64>());
        let mut cnt = 0;
        let mut x = 1;
        while x*x < w {
            let y = (((w - x*x) as f64).sqrt() + 1e-6) as i64;
            if y*y != w - x*x { x += 1; continue; }
            if x * y1 >= y * x1 && x * y2 <= y * x2 { cnt += 1; }
            x += 1;
        }
        println!("{}", cnt);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}