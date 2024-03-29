// BOJ 30428 [Reversal]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, m, k, t) = (next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>());
    let mut v = [false; 1<<24];
    for a in 0..n { for b in 0..m { for c in 0..k { for d in 0..t {
        v[a<<18|b<<12|c<<6|d] = next::<u8>() == 1;
    }}}}

    let mut p = vec![0; 1<<24];
    for a in 1..=n { for b in 1..=m { for c in 1..=k { for d in 1..=t {
        p[a<<18|b<<12|c<<6|d] = v[a-1<<18|b-1<<12|c-1<<6|d-1] as i32
            + p[a-1<<18|b<<12|c<<6|d] + p[a<<18|b-1<<12|c<<6|d] + p[a<<18|b<<12|c-1<<6|d] + p[a<<18|b<<12|c<<6|d-1]
            - p[a-1<<18|b-1<<12|c<<6|d] - p[a-1<<18|b<<12|c-1<<6|d] - p[a-1<<18|b<<12|c<<6|d-1] - p[a<<18|b-1<<12|c-1<<6|d] - p[a<<18|b-1<<12|c<<6|d-1] - p[a<<18|b<<12|c-1<<6|d-1]
            + p[a-1<<18|b-1<<12|c-1<<6|d] + p[a-1<<18|b-1<<12|c<<6|d-1] + p[a-1<<18|b<<12|c-1<<6|d-1] + p[a<<18|b-1<<12|c-1<<6|d-1]
            - p[a-1<<18|b-1<<12|c-1<<6|d-1];
    }}}}

    for x in (1..=n.min(m).min(t).min(k)).rev() {
        for a in 0..=n-x { for b in 0..=m-x { for c in 0..=k-x { for d in 0..=t-x {
            if p[a+x<<18|b+x<<12|c+x<<6|d+x] - p[a+x<<18|b+x<<12|c+x<<6|d] - p[a+x<<18|b+x<<12|c<<6|d+x] - p[a+x<<18|b<<12|c+x<<6|d+x] - p[a<<18|b+x<<12|c+x<<6|d+x]
                + p[a+x<<18|b+x<<12|c<<6|d] + p[a+x<<18|b<<12|c+x<<6|d] + p[a+x<<18|b<<12|c<<6|d+x] + p[a<<18|b+x<<12|c<<6|d+x] + p[a<<18|b+x<<12|c+x<<6|d] + p[a<<18|b<<12|c+x<<6|d+x]
                - p[a+x<<18|b<<12|c<<6|d] - p[a<<18|b+x<<12|c<<6|d] - p[a<<18|b<<12|c+x<<6|d] - p[a<<18|b<<12|c<<6|d+x] + p[a<<18|b<<12|c<<6|d] == 0 {
                println!("{}", x);
                return;
            }
        }}}}
    }
    println!("0");
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