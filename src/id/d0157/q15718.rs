// BOJ 15718 [Tteokfire Returns]

fn gen(fmod: &mut Vec<i64>, finv: &mut Vec<i64>, max: usize, m: i64) {
    let mut dp = vec![1; max];
    for i in 2..max {
        dp[i] = -(m / i as i64) * dp[m as usize % i] % m;
        fmod[i] = fmod[i-1] * i as i64 % m;
        finv[i] = finv[i-1] * dp[i] % m;
    }
}
#[inline] fn ncr(n: usize, r: usize, fmod: &[i64], finv: &[i64], m: i64) -> i64 {
    if n < r { return 0 }
    fmod[n] * finv[r] % m * finv[n-r] % m
}
fn xgcd(a: i64, b: i64) -> (i64, i64, i64) {
    return if b == 0 { (a, 1, 0) }
    else {
        let (g, x, y) = xgcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}
fn crt(a: i64, b: i64, m: i64, n: i64) -> i64 {
    let (_, x, y) = xgcd(m, n);
    let k = m * n;
    (a * n % k * y % k + b * m % k * x % k + k) % k
}
pub fn main() { read();
    let (m1, m2) = (97, 1031);
    let mut fmod1 = vec![1; m1];
    let mut finv1 = vec![1; m1];
    gen(&mut fmod1, &mut finv1, m1, m1 as i64);
    let mut fmod2 = vec![1; m2];
    let mut finv2 = vec![1; m2];
    gen(&mut fmod2, &mut finv2, m2, m2 as i64);

    for _ in 0..next() {
        let (n, m) = (next::<usize>(), next::<usize>());
        if n == 0 { println!("{}", if m == 1 { 1 } else { 0 }); continue; }
        if m == 1 { println!("0"); continue; }
        if n < m - 1 { println!("0"); continue; }

        let mut r1 = 1;
        let (mut a, mut b) = (n as i64 - 1, m as i64 - 2);
        while a > 0 || b > 0 {
            r1 = r1 * ncr(a as usize % m1, b as usize % m1, &fmod1, &finv1, m1 as i64) % m1 as i64;
            a /= m1 as i64;
            b /= m1 as i64;
        }
        r1 = (r1 + m1 as i64) % m1 as i64;
        let mut r2 = 1;
        let (mut a, mut b) = (n as i64 - 1, m as i64 - 2);
        while a > 0 || b > 0 {
            r2 = r2 * ncr(a as usize % m2, b as usize % m2, &fmod2, &finv2, m2 as i64) % m2 as i64;
            a /= m2 as i64;
            b /= m2 as i64;
        }
        r2 = (r2 + m2 as i64) % m2 as i64;

        println!("{}", crt(r1, r2, m1 as i64, m2 as i64));
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