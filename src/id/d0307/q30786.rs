// BOJ 30786 [Odd Travel]
// Supported by GitHub Copilot

pub fn main() {
    read();
    let n: usize = next();
    let (mut a, mut b) = (vec![], vec![]);
    for i in 1..=n {
        match (next::<i32>() + next::<i32>()) % 2 {
            0 => a.push(i),
            _ => b.push(i),
        }
    }

    if a.is_empty() || b.is_empty() { println!("NO"); }
    else {
        println!("YES");
        a.iter().for_each(|x| print!("{} ", x));
        b.iter().for_each(|x| print!("{} ", x));
        println!();
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