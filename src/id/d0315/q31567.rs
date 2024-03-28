// BOJ 31567 [Move or Block!]
pub fn main() { read();
    let n = next::<usize>();
    let mut b = vec![b'X'; n+2];
    next::<String>().into_bytes().into_iter().enumerate().for_each(|(i, c)| b[i+1] = c);

    let i = b.iter().position(|&c| c == b'O').unwrap();
    if b[i-1] == b'X' || b[i+1] == b'X' { println!("mingyu"); return; }
    if b[i-2] != b'X' && b[i+2] != b'X' { println!("draw"); return; }
    let c = b.iter().filter(|&&c| c == b'.').count();
    if b[i-2] == b'X' && b[i+2] == b'X' {
        println!("{}", if c & 1 == 1 { "mingyu" } else { "yunsu" });
    } else {
        println!("{}", if c & 1 == 1 { "mingyu" } else { "draw" });
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}