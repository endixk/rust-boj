// BOJ 30623 [Permutation and Operations]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>(); let m = n-1;
    let a = (0..n).map(|_| next::<usize>()).collect::<Vec<_>>();
    if a[0] < a[1] && a[1] < a[n-1] { println!("4\n1 1\n2 1 {n}\n1 1\n2 1 {n}"); }
    else if a[1] < a[0] && a[0] < a[n-1] { println!("3\n2 1 {n}\n1 1\n2 1 {n}"); }
    else if a[n-1] < a[0] && a[0] < a[1] { println!("3\n2 1 {n}\n1 1\n2 1 {n}"); }
    else if a[n-1] < a[1] && a[1] < a[0] { println!("4\n1 1\n2 1 {n}\n1 1\n2 1 {n}"); }
    else if a[0] < a[n-1] && a[n-1] < a[1] && a[1] < a[n-2] { println!("3\n2 1 {n}\n1 {m}\n2 1 {n}"); }
    else if a[0] < a[n-1] && a[n-1] < a[n-2] && a[n-2] < a[1] { println!("3\n2 1 {n}\n1 {m}\n2 1 {n}"); }
    else if a[0] < a[n-2] && a[n-2] < a[n-1] && a[n-1] < a[1] { println!("4\n1 1\n2 1 {n}\n1 {m}\n2 1 {n}"); }
    else if a[n-2] < a[0] && a[0] < a[n-1] && a[n-1] < a[1] { println!("4\n1 1\n2 1 {n}\n1 {m}\n2 1 {n}"); }
    else if a[1] < a[n-1] && a[n-1] < a[0] && a[0] < a[n-2] { println!("4\n1 {m}\n2 1 {n}\n1 1\n2 1 {n}"); }
    else if a[1] < a[n-1] && a[n-1] < a[n-2] && a[n-2] < a[0] { println!("4\n1 {m}\n2 1 {n}\n1 {m}\n2 1 {n}"); }
    else { println!("3\n2 1 {n}\n1 {m}\n2 1 {n}"); }
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