// BOJ 30826 [That Long Number]
// Supported by GitHub Copilot

pub fn main() { read();
    let mut k = next::<usize>();
    let mut i = 0;
    while k > 9 * (i+1) * 10usize.pow(i as u32 / 2) {
        k -= 9 * (i+1) * 10usize.pow(i as u32 / 2);
        i += 1;
    }

    let n = 10usize.pow(i as u32 / 2) + (k-1) / (i+1);
    let m = (k-1) % (i+1);
    let m = if i-m <= m { i-m } else { m };
    println!("{}", n.to_string().chars().nth(m).unwrap());
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