// BOJ 30430 [Reversi]
// Supported by GitHub Copilot

pub fn main() { read();
    for _ in 0..next() {
        let (n, m) = (next::<usize>(), next::<usize>());
        let (mut b, mut w) = (0, 0);
        for i in 0..n { for j in 0..m {
            if (i + j) & 1 == 1 { b += 1; } else { w += 1; }
        }}
        let (mut x, mut y) = (0, 0);
        while b > 0 && w > 0 {
            x += 2; b -= 1; w -= 1;
            if b > 0 { y += 1; b -= 1; }
        }
        y += b + w;
        print!("{}", if x > y { "L" } else if x < y { "W" } else { "D" });
    }
    println!();
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