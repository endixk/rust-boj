// BOJ 28961 [Doctor Strange and the Exhibition]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, k) = (next::<usize>(), next::<usize>());
    let mut a = (0..n).map(|_| next::<u16>()).collect::<Vec<_>>();

    let (mut x, mut c) = (0xf000u16, 0);
    while x < 0xffff {
        if a.is_empty() { println!("NO"); return; }
        a.sort_unstable_by(|&a, &b| ((a|x).count_zeros()).cmp(&(b|x).count_zeros()));
        let k = a.pop().unwrap();
        if (k|x).count_zeros() == 0 { println!("NO"); return; }
        x |= !k; c += 1;
    }
    println!("{}", if c > k { "NO" } else { "YES" });
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