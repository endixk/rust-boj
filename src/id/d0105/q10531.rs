// BOJ 10531 [Golf Bot]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;
use std::ops::{Add, Sub, Mul, Div};

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

#[derive(Debug, Clone, Copy)]
struct Complex {
    r: f64, i: f64
}
impl Complex {
    fn default() -> Self {
        Self { r: 0.0, i: 0.0 }
    }
    fn new(r: f64, i: f64) -> Self {
        Self { r, i }
    }
}
impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.i + rhs.i)
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.r - rhs.r, self.i - rhs.i)
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.r * rhs.r - self.i * rhs.i, self.r * rhs.i + self.i * rhs.r)
    }
}
impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let d = rhs.r * rhs.r + rhs.i * rhs.i;
        Self::new((self.r * rhs.r + self.i * rhs.i) / d, (self.i * rhs.r - self.r * rhs.i) / d)
    }
}

const PI: f64 = std::f64::consts::PI;
fn fft(v: &mut Vec<Complex>, w: Complex) {
    let n = v.len();
    if n == 1 { return; }

    let (mut ev, mut ov) = (vec![Complex::default(); n>>1], vec![Complex::default(); n>>1]);
    for i in 0..n {
        if i & 1 == 0 { ev[i>>1] = v[i]; }
        else { ov[i>>1] = v[i]; }
    }

    fft(&mut ev, w * w);
    fft(&mut ov, w * w);

    let mut p = Complex::new(1.0, 0.0);
    for i in 0..(n>>1) {
        v[i] = ev[i] + p * ov[i];
        v[i + (n>>1)] = ev[i] - p * ov[i];
        p = p * w;
    }
}

fn multiply(mut a: Vec<Complex>, mut b: Vec<Complex>) -> Vec<Complex> {
    let mut n = 1;
    while n < a.len() + b.len() { n <<= 1; }
    a.resize(n, Complex::default());
    b.resize(n, Complex::default());

    let w = Complex::new((PI * 2.0 / n as f64).cos(), (PI * 2.0 / n as f64).sin());
    fft(&mut a, w);
    fft(&mut b, w);

    for i in 0..n { a[i] = a[i] * b[i]; }
    fft(&mut a, Complex::new(1.0, 0.0) / w);

    for i in 0..n { a[i] = a[i] / Complex::new(n as f64, 0.0); }
    a
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut a = vec![Complex::default(); 200001];
    for _ in 0..next::<usize>(&mut it) {
        a[next::<usize>(&mut it)] = Complex::new(1.0, 0.0);
    }

    let (b, c) = (a.clone(), a.clone());
    a = multiply(a, b);

    let mut ans = 0;
    for _ in 0..next::<usize>(&mut it) {
        let x = next::<usize>(&mut it);
        if a[x].r > 0.5 { ans += 1; }
        else if c[x].r > 0.5 { ans += 1; }
    }
    println!("{}", ans);
}
