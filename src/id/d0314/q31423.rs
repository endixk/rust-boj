// BOJ 31423 [Sinchon Merger]
pub fn main() { read();
    let n = next::<usize>();
    let a = (0..n).map(|_| next::<String>()).collect::<Vec<_>>();
    let mut ll = [0usize; 500000];
    for i in 0..n { ll[i] = i<<32|i; }

    let mut st = 0;
    for _ in 1..n {
        let (i, j) = (next::<usize>() - 1, next::<usize>() - 1);
        let (it, jt) = (ll[i]&0xffffffff, ll[j]&0xffffffff);
        ll[i] &= 0xffffffff00000000; ll[i] |= jt;
        ll[it] = j<<32|jt;
        st = i;
    }

    for _ in 0..n {
        print!("{}", a[st]);
        st = ll[st]>>32;
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