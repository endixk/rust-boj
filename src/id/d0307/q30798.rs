pub fn main() { read();
    let (n, x) = (next::<usize>(), next::<usize>());
    let mut k = 0;
    let mut v = vec![];
    for i in 1..n-1 {
        v.push(i);
        k ^= i;
    }
    if k == x {
        let y = v.pop().unwrap();
        v.push((1<<40) | y);
        v.push((1<<60) | (1<<40) | k);
        v.push((1<<60) | x);
    } else {
        v.push((1<<60) + k);
        v.push((1<<60) + x);
    }
    v.sort_unstable();

    println!("{}", n);
    for i in (0..n).rev() {
        println!("{}", v[i]);
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