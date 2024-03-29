// BOJ 6565 [Hard to Believe, but True!]
pub fn main() { read();
    loop {
        let mut s = next::<String>().into_bytes();
        s.reverse();
        let (i, j) = (s.iter().position(|&c| c == b'=').unwrap(), s.iter().position(|&c| c == b'+').unwrap());
        let c = u32::from_str_radix(&String::from_utf8_lossy(&s[..i]), 10).unwrap();
        let a = u32::from_str_radix(&String::from_utf8_lossy(&s[i+1..j]), 10).unwrap();
        let b = u32::from_str_radix(&String::from_utf8_lossy(&s[j+1..]), 10).unwrap();
        if a == 0 && b == 0 && c == 0 {
            println!("True");
            break;
        }
        println!("{}", if a + b == c { "True" } else { "False" });
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