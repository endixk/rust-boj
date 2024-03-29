// BOJ 31421 [Flipping Hotteoks]
pub fn main() { read();
    let n = next::<usize>();
    let s = next::<String>().into_bytes();
    let mut p = (1..n).filter(|&i| s[i-1] != s[i]).map(|i| i as u32).collect::<Vec<_>>();
    if p.len() == 0 {
        if s[0] == b'W' { println!("0"); }
        else { println!("-1"); }
    } else if p.len() == 1 {
        if s[0] == b'W' { println!("-1"); }
        else { println!("1\n{}", p[0]); }
    } else {
        println!("{}", p.len());
        if (p.len() & 1 == 0 && s[0] == b'B') || (p.len() & 1 == 1 && s[0] == b'W') {
            let a = p.pop().unwrap();
            let b = p.pop().unwrap();
            p.push(a); p.push(b);
        }
        p.into_iter().for_each(|x| println!("{}", x));
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