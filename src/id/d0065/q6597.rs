// BOJ 6597 [Tree Recovery]
fn recover(s: &[u8], t: &[u8], i: &mut usize, p: usize, q: usize) {
    if p > q { return; }
    let c= s[*i]; *i += 1;
    if p == q { print!("{}", c as char); return; }
    let r = t.iter().position(|&x| x == c).unwrap();
    recover(s, t, i, p, r-1);
    recover(s, t, i, r+1, q);
    print!("{}", c as char);
}
pub fn main() { read();
    while peek() {
        let mut i = 0;
        let (s, t) = (next::<String>().into_bytes(), next::<String>().into_bytes());
        recover(&s, &t, &mut i, 0, s.len()-1);
        println!();
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, iter::Peekable};
static mut BUF: String = String::new();
static mut IT: Option<Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }
