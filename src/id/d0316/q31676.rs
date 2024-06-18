// BOJ 31676 [Exceptional Cake]
pub fn main() { read();
    for _ in 0..4 { next::<String>(); }
    let n = next::<usize>();
    let mut a = vec![0; n+1];
    let mut b = vec![0; n+1];
    let mut q = vec![];
    let mut d = 0;
    for _ in 0..n {
        let m = next::<usize>();
        let k = (0..m).map(|_| next::<usize>()).collect::<Vec<_>>();
        let f = next::<u8>() == 1;
        if f {
            d += 1;
            for &i in &k { a[i] += 1; }
        } else {
            for &i in &k { b[i] += 1; }
        }
        q.push((f, k));
    }

    let mut c = vec![];
    for (i, (f, k)) in q.into_iter().enumerate() {
        // flip the testimony
        if f {
            d -= 1;
            for &i in &k { a[i] -= 1; b[i] += 1; }
        } else {
            d += 1;
            for &i in &k { a[i] += 1; b[i] -= 1; }
        }

        if a[i+1] == d && b[i+1] == 0 { c.push(i+1); }

        // flip back
        if f {
            d += 1;
            for &i in &k { a[i] += 1; b[i] -= 1; }
        } else {
            d -= 1;
            for &i in &k { a[i] -= 1; b[i] += 1; }
        }
    }

    if c.len() == 0 { print!("swi"); }
    else { c.iter().for_each(|&x| print!("{} ", x)); }
    println!();
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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