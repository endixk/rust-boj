// BOJ 30963 [Claw Machine]
// Supported by GitHub Copilot

fn pow(mut a: i64, mut p: i64, m: i64) -> i64 {
    let mut ret = 1;
    while p > 0 {
        if p & 1 > 0 { ret = ret * a % m; }
        a = a * a % m;
        p >>= 1;
    }
    ret
}
pub fn main() { read();
    const MOD: i64 = 1_000_000_007;
    for _ in 0..next() {
        let (p, c, n) = (next::<i64>(), next::<usize>(), next::<usize>());
        if c > n {
            for i in 1..=n {
                println!("{}", p * i as i64 % MOD);
            }
            continue;
        }
        let mut d = vec![0; n+1];
        let k = pow((MOD - p + 1) % MOD, c as i64, MOD);
        for i in 1..c {
            d[i] = p * i as i64 % MOD;
            println!("{}", d[i]);
        }
        d[c] = (d[c-1] + p + k) % MOD;
        println!("{}", d[c]);
        for i in c+1..=n {
            d[i] = (d[i-1] + p + (d[i-c] - d[i-c-1]) * k % MOD + MOD) % MOD;
            println!("{}", d[i]);
        }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}