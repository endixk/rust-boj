// BOJ 13140 [Hello World!]
fn go(v: &mut [u32; 7], b: &mut [bool; 10], c: usize, n: u32) {
    if c == 7 {
        let x = v[0] + v[1] * 1000 + v[2] * 10000 + v[3] * 120 + v[4] * 1001 + v[5] * 100 + v[6] * 10000;
        if x == n {
            println!("  {}{}{}{}{}", v[2], v[1], v[3], v[3], v[4]);
            println!("+ {}{}{}{}{}", v[6], v[4], v[5], v[3], v[0]);
            println!("-------");
            println!(" {:6}", n);
            std::process::exit(0);
        } else { return; }
    }
    for i in 0..10 {
        if b[i] { continue; }
        if i == 0 && (c == 2 || c == 6) { continue; }
        v[c] = i as u32; b[i] = true;
        go(v, b, c + 1, n);
        b[i] = false;
    }
}
pub fn main() { read();
    let n = next::<u32>();
    let mut v = [0; 7];
    let mut b = [false; 10];
    go(&mut v, &mut b, 0, n);
    println!("No Answer");
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
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