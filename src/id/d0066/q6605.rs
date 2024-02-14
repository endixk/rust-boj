// BOJ 6605 [Humble Numbers]
const MAX: u64 = 2_000_000_000;
pub fn main() { read();
    let mut v = vec![1];
    while v.len() < 5842 {
        let mut nv = vec![];
        for &x in &v {
            if x * 2 <= MAX { nv.push(x * 2); }
            if x * 3 <= MAX { nv.push(x * 3); }
            if x * 5 <= MAX { nv.push(x * 5); }
            if x * 7 <= MAX { nv.push(x * 7); }
        }
        v.extend(nv);
        v.sort_unstable();
        v.dedup();
    }

    loop {
        let n = next::<usize>();
        if n == 0 { break; }
        match n % 10 {
            1 if n % 100 != 11 => println!("The {}st humble number is {}.", n, v[n-1]),
            2 if n % 100 != 12 => println!("The {}nd humble number is {}.", n, v[n-1]),
            3 if n % 100 != 13 => println!("The {}rd humble number is {}.", n, v[n-1]),
            _ => println!("The {}th humble number is {}.", n, v[n-1]),
        }
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