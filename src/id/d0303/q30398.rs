// BOJ 30398 [DG1S]
// Supported by GitHub Copilot

const MOD: i64 = 1_000_000_007;
const MAX: usize = 1_000_001;
pub fn main() { read();
    let mut d = vec![1; MAX];
    let mut fmod = vec![1; MAX];
    let mut finv = vec![1; MAX];
    for i in 2..MAX {
        d[i] = -(MOD / i as i64) * d[MOD as usize % i] % MOD;
        fmod[i] = fmod[i-1] * i as i64 % MOD;
        finv[i] = finv[i-1] * d[i] % MOD;
    }

    let (n, m, k) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut v = (0..m).map(|_| 1).collect::<Vec<_>>();
    let mut ans = 1;
    for _ in 0..k {
        let mut c = 0;
        for i in 0..m {
            let x = next::<usize>();
            ans = ans * finv[x - v[i]] % MOD;
            c += x - v[i];
            v[i] = x;
        }
        ans = ans * fmod[c] % MOD;
    }
    let mut c = 0;
    for i in 0..m {
        ans = ans * finv[n - v[i]] % MOD;
        c += n - v[i];
    }
    ans = ans * fmod[c] % MOD;

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