// BOJ 30629 [A Boring Question]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    if m < n {
        for _ in 0..m { print!("1 "); }
        for _ in m..n { print!("0 "); }
        println!();
    } else {
        let mut v = vec![0; n];
        let k = n/2;
        for i in 0..k-1 { v[i] = 1; }

        let (mut i, mut x) = (k-1, k-1);
        while x + k <= m {
            v[i] = k;
            x += k;
            i += 1;
        }
        v[i] = m - x;

        for i in 0..n { print!("{} ", v[i]); }
        println!();
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