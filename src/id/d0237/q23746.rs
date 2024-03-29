// BOJ 23746 [String Decompression]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut m = vec![String::new(); 26];
    for _ in 0..n {
        let (s, c) = (next::<String>(), next::<char>());
        m[(c as u8 - b'A') as usize] = s;
    }

    let s = next::<String>();
    let mut t = String::new();
    for c in s.chars() {
        t += &m[(c as u8 - b'A') as usize];
    }

    let (l ,r) = (next::<usize>(), next::<usize>());
    println!("{}", &t[l-1..r]);
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