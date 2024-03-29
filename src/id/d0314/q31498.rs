// BOJ 31498 [Mischievous Miss Toca]
fn go(a: i128, b: i128, k: i128, x: i128) -> bool {
    let x = if b < x*k { b / k } else { x };
    b * x - k * x * (x - 1) / 2 >= a
}
pub fn main() { read();
    let (a, b) = (next::<i128>(), next::<i128>());
    let (c, d) = (next::<i128>(), next::<i128>());
    let k = next::<i128>();
    let (mut l, mut r) = (1, 2_000_000_000_000);
    while l < r {
        let m = (l + r) / 2;
        if go(a, b, k, m) { r = m; } else { l = m + 1; }
    }
    println!("{}", if l < (a + c + d - 1) / d { l } else { -1 });
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