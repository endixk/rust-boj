// BOJ 30844 [Fruitful Collaboration]
// Supported by GitHub Copilot

#[inline] fn nkd(ten: &[u64], n: u64, k: usize, d: u64) -> u64 {
    if d == 0 && n / ten[k+1] == 0 { return 0; }
    n / ten[k+1] * ten[k] +
        if n / ten[k] % 10 > d { ten[k] }
        else if n / ten[k] % 10 == d { n % ten[k] + 1 }
        else { 0 } -
        if d == 0 { ten[k] } else { 0 }
}
pub fn main() { read();
    let n = next::<u64>();
    let ten = (0..20).map(|i| 10u64.pow(i)).collect::<Vec<_>>();
    let mut ans = 0;
    for i in 0..16 {
        ans += nkd(&ten, n, i, 0) + nkd(&ten, n, i, 6) + 2 * nkd(&ten, n, i, 8) + nkd(&ten, n, i, 9);
    }
    println!("{}", ans);
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