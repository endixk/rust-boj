// BOJ 20176 [Needle]
fn pow(mut a: i64, mut b: i64, m: i64) -> i64 {
    let mut r = 1;
    while b > 0 {
        if b & 1 == 1 {
            r = r * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }
    r
}
fn ntt(v: &mut Vec<i64>, m: i64, w: i64, inv: bool) {
    let n = v.len();
    let mut i = 0;
    for j in 1..n - 1 {
        let mut k = n >> 1;
        while i & k > 0 {
            i ^= k;
            k >>= 1;
        }
        i ^= k;
        if i < j {
            v.swap(i, j);
        }
    }
    let mut step = 1;
    while step < n {
        let mut root = pow(w, (m - 1) / (step << 1) as i64, m);
        if inv {
            root = pow(root, m - 2, m);
        }
        for i in (0..n).step_by(step << 1) {
            let mut w = 1;
            for j in 0..step {
                let x = v[i + j];
                let y = v[i + j + step] * w % m;
                v[i + j] = (x + y) % m;
                v[i + j + step] = (x - y + m) % m;
                w = w * root % m;
            }
        }
        step <<= 1;
    }
    if inv {
        let inv = pow(n as i64, m - 2, m);
        for i in 0..n {
            v[i] = v[i] * inv % m;
        }
    }
}
fn multiply(mut a: Vec<i64>, mut b: Vec<i64>, m: i64, w: i64) -> Vec<i64> {
    let n = (a.len() + b.len()).next_power_of_two();
    a.resize(n, 0);
    b.resize(n, 0);
    ntt(&mut a, m, w, false);
    ntt(&mut b, m, w, false);
    for i in 0..n {
        a[i] = a[i] * b[i] % m;
    }
    ntt(&mut a, m, w, true);
    a
}
pub fn main() { read();
    let mut a = vec![0; 60001];
    for _ in 0..next() { a[(next::<i32>() + 30000) as usize] = 1; }

    let n = next::<usize>();
    let q = (0..n).map(|_| (next::<i32>() + 30000 << 1) as usize).collect::<Vec<_>>();

    let mut b = vec![0; 60001];
    for _ in 0..next() { b[(next::<i32>() + 30000) as usize] = 1; }

    let m = multiply(a, b, 998244353, 3);
    println!("{}", q.iter().map(|&x| m[x]).sum::<i64>());
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