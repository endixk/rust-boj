// BOJ 27920 [Card Flip]
pub fn main() { read();
    let n = next::<usize>();
    println!("YES");
    if n == 1 { println!("1\n1"); return; }

    let mut x = n - n%2;
    for _ in 0..n {
        print!("{} ", x);
        x = match x % 2 {
            0 if x == 2 => 1,
            0 => x - 2,
            _ => x + 2,
        }
    }
    println!();

    let (mut x, mut y) = (n/2+1, n/2);
    for i in 0..n {
        match i % 2 {
            0 => { print!("{} ", x); x += 1; }
            _ => { print!("{} ", y); y -= 1; }
        }
    }
    println!();
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