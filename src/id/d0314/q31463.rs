// BOJ 31463 [Chocolate Flipping Game (Sweet)]
const MOD: u64 = 1_000_000_007;
fn pmod(a: u64, p: u64, m: u64) -> u64 {
    let (mut r, mut a, mut p) = (1, a, p);
    while p > 0 {
        if p & 1 == 1 { r = r * a % m; }
        a = a * a % m;
        p >>= 1;
    }
    r
}
pub fn main() { read();
    for _ in 0..next() {
        let (mut h, mut k) = (0, 0);
        for c in next::<String>().chars() {
            match c { 'H' => h += 1, '?' => k += 1, _ => {} }
        }
        if k == 0 { println!("{}", h & 1); }
        else { println!("{}", pmod(2, k-1, MOD)); }
    }
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