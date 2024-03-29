pub fn main() { read();
    let n = next::<usize>();
    let mut a = (0..n).map(|_| next::<i64>()).collect::<Vec<_>>();
    a.sort_unstable();
    let mut d = vec![0x3f3f3f3f; n];
    let mut e = vec![0x3f3f3f3f; n];
    d[1] = a[1] - a[0]; e[2] = a[2] - a[0];
    for i in 3..n {
        if i % 2 == 1 {
            d[i] = d[i-2] + a[i] - a[i-1];
        } else {
            e[i] = (e[i-2] + a[i] - a[i-1]).min(d[i-3] + a[i] - a[i-2]);
        }
    }
    println!("{}", e[n-1]);
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