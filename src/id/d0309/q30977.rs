// BOJ 30977 [Eating Comes First]
fn pow(mut a: u64, mut b: u64, m: u64) -> u64 {
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

fn ntt(v: &mut Vec<u64>, m: u64, w: u64, inv: bool) {
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
        let mut root = pow(w, (m - 1) / (step << 1) as u64, m);
        if inv {
            root = pow(root, m - 2, m);
        }
        for i in (0..n).step_by(step << 1) {
            let mut w = 1;
            for j in 0..step {
                let x = v[i + j];
                let y = v[i + j + step] * w % m;
                v[i + j] = (x + y) % m;
                v[i + j + step] = (x + m - y) % m;
                w = w * root % m;
            }
        }
        step <<= 1;
    }
    if inv {
        let inv = pow(n as u64, m - 2, m);
        for i in 0..n {
            v[i] = v[i] * inv % m;
        }
    }
}

fn multiply(mut a: Vec<u64>, mut b: Vec<u64>, m: u64, w: u64) -> Vec<u64> {
    let n = (a.len() + b.len()).next_power_of_two();
    a.resize(n, 0);
    b.resize(n, 0);
    ntt(&mut a, m, w, false);
    ntt(&mut b, m, w, false);
    for i in 0..n {
        a[i] = a[i] * b[i] % m;
    }
    ntt(&mut a, m, w, true);
    a = a.into_iter().map(|x| if x > 0 { 1 } else { 0 }).collect();
    let i = a.iter().rposition(|&x| x > 0).unwrap();
    a.truncate(i + 1);
    a
}
fn power(mut a: Vec<u64>, mut p: u32) -> Vec<u64> {
    let mut ret = vec![1];
    while p > 0 {
        if p & 1 == 1 { ret = multiply(ret, a.clone(), 998244353, 3); }
        a = multiply(a.clone(), a, 998244353, 3);
        p >>= 1;
    }
    ret
}
fn pi(s: &[bool]) -> Vec<usize> {
    let l = s.len();
    let mut p = vec![0; l];

    let mut j = 0;
    for i in 1..l {
        while j > 0 && s[i] != s[j] { j = p[j-1]; }
        if s[i] == s[j] { j += 1; p[i] = j; }
    }

    p
}
fn kmp(s: &[bool], t: &[bool]) -> u32 {
    let p = pi(t);
    let (n, m) = (s.len(), t.len());

    let (mut occ, mut j) = (0, 0);
    for i in 0..n {
        while j > 0 && s[i] != t[j] { j = p[j-1]; }
        if s[i] == t[j] {
            if j == m-1 { occ += 1; j = p[j]; }
            else { j += 1; }
        }
    }
    occ
}
pub fn main() { read();
    let (n, m, k) = (next::<usize>(), next::<u32>(), next::<usize>());
    let mut a = vec![0; 1001];
    for _ in 0..n { a[next::<usize>()] = 1; }
    let x = power(a, m).into_iter().map(|x| x > 0).collect::<Vec<_>>();

    let mut d = vec![];
    let mut i = 0;
    for _ in 0..k {
        let t = next::<usize>();
        while i < t { d.push(false); i += 1; }
        d.push(true); i += 1;
    }

    println!("{}", kmp(&x[1..], &d));
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