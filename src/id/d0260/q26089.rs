// BOJ 26089 [Perfect Sequence Reverse]
fn suffix(n: usize, m: usize) {
    if n > m && n > 4 {
        print!("4 "); suffix(n-1, m); return;
    }
    match (n, m) {
        (4, 0) => println!("4 4 4 4"),
        (4, 1) => println!("4 4 2 0"),
        (4, 2) => println!("4 0 0 2"),
        (4, 3) => println!("4 0 2 0"),
        (4, 4) => println!("2 0 0 2"),
        (5, 5) => println!("2 0 2 0 2"),
        (6, 6) => println!("4 2 0 0 2 0"),
        (7, 7) => println!("4 2 0 2 0 2 0"),
        _ => (),
    }
}
pub fn main() { read();
    let (mut n, mut m) = (next::<usize>(), next::<usize>());
    while n > 7 {
        if m > 3 { print!("2 0 0 2 "); m -= 4; }
        else { print!("4 4 4 4 "); }
        n -= 4;
    }
    suffix(n, m);
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}