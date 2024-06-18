// BOJ 31677 [Exceptional Drop-kick 2]
pub fn main() { read();
    let (_, mut m) = (next::<usize>(), next::<i32>());
    let mut v = vec![];
    let (mut w, mut c, mut k, mut p) = (0, 0, 0, '.');
    for x in next::<String>().chars() {
        match x {
            'X' => {
                if p == '.' { c += 1; v.push(k) }
                k = 0;
            }
            _ => { w += 1; k += 1; }
        }
        p = x;
    }
    if v.is_empty() { println!("{}", w + 2 * c); return; }

    v.remove(0);
    v.sort_unstable();
    for x in v {
        if m < x { break; }
        m -= x; c -= 1;
    }

    println!("{}", w + 2 * c);
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