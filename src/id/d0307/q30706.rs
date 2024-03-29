// BOJ 30706 [Organizing KCPC]
// Supported by GitHub Copilot

pub fn main() { read();
    const MOD: i64 = 1_000_000_007;
    let (n, m, k) = (next::<i64>(), next::<i64>(), next::<i64>());

    let (mut nxs, mut nys, mut nxq, mut nyq) = (0, 0, 0, 0);
    for _ in 0..n {
        let (x, y) = (next::<i64>(), next::<i64>());
        nxs = (nxs + x) % MOD; nxq = (nxq + x * x) % MOD;
        nys = (nys + y) % MOD; nyq = (nyq + y * y) % MOD;
    }
    let (mut mxs, mut mys, mut mxq, mut myq) = (0, 0, 0, 0);
    for _ in 0..m {
        let (x, y) = (next::<i64>(), next::<i64>());
        mxs = (mxs + x) % MOD; mxq = (mxq + x * x) % MOD;
        mys = (mys + y) % MOD; myq = (myq + y * y) % MOD;
    }
    let (mut kxs, mut kys, mut kxq, mut kyq) = (0, 0, 0, 0);
    for _ in 0..k {
        let (x, y) = (next::<i64>(), next::<i64>());
        kxs = (kxs + x) % MOD; kxq = (kxq + x * x) % MOD;
        kys = (kys + y) % MOD; kyq = (kyq + y * y) % MOD;
    }

    let mut ans = 0;
    // n -> m, k
    ans = (ans + 2 * nxq * m % MOD * k % MOD) % MOD;
    ans = (ans - 2 * nxs * mxs % MOD * k % MOD) % MOD;
    ans = (ans + 2 * nyq * m % MOD * k % MOD) % MOD;
    ans = (ans - 2 * nys * mys % MOD * k % MOD) % MOD;
    ans = (ans + nxs * mys % MOD * k % MOD) % MOD;
    ans = (ans - nys * mxs % MOD * k % MOD) % MOD;
    // m -> k, n
    ans = (ans + 2 * mxq * k % MOD * n % MOD) % MOD;
    ans = (ans - 2 * mxs * kxs % MOD * n % MOD) % MOD;
    ans = (ans + 2 * myq * k % MOD * n % MOD) % MOD;
    ans = (ans - 2 * mys * kys % MOD * n % MOD) % MOD;
    ans = (ans + mxs * kys % MOD * n % MOD) % MOD;
    ans = (ans - mys * kxs % MOD * n % MOD) % MOD;
    // k -> n, m
    ans = (ans + 2 * kxq * n % MOD * m % MOD) % MOD;
    ans = (ans - 2 * kxs * nxs % MOD * m % MOD) % MOD;
    ans = (ans + 2 * kyq * n % MOD * m % MOD) % MOD;
    ans = (ans - 2 * kys * nys % MOD * m % MOD) % MOD;
    ans = (ans + kxs * nys % MOD * m % MOD) % MOD;
    ans = (ans - kys * nxs % MOD * m % MOD) % MOD;

    println!("{}", (ans + MOD) % MOD);

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