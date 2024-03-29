// BOJ 13713 [String and Queries]
fn z(s: &String) -> Vec<u32> {
    let (s, n) = (s.as_bytes(), s.len());
    let mut z = vec![0; n];
    z[n-1] = n as u32;

    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i > r {
            l = i; r = i;
            while r < n && s[n-1+l-r] == s[n-1-r] { r += 1; }
            z[n-1-i] = (r-l) as u32; r -= 1;
        } else {
            let k = i-l;
            if z[n-1-k] < (r-i+1) as u32 { z[n-1-i] = z[n-1-k]; }
            else {
                l = i;
                while r < n && s[n-1+l-r] == s[n-1-r] { r += 1; }
                z[n-1-i] = (r-l) as u32; r -= 1;
            }
        }
    }

    z
}
pub fn main() { read();
    let z = z(&next::<String>());
    for _ in 0..next() {
        println!("{}", z[next::<usize>()-1]);
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