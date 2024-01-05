use std::ops::{Add, Sub, Mul, Div};
use std::f64::consts::PI;
#[derive(Debug, Clone, Copy)] struct Complex { r: f64, i: f64 }
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
    let n = (a.len() + b.len()).next_power_of_two();
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
fn poly_multiply(a: Vec<i64>, b: Vec<i64>) -> Vec<i64> {
    let a = a.iter().map(|&x| Complex::new(x as f64, 0.0)).collect::<Vec<_>>();
    let b = b.iter().map(|&x| Complex::new(x as f64, 0.0)).collect::<Vec<_>>();
    let c = multiply(a, b);
    c.iter().map(|&x| x.r.round() as i64).collect::<Vec<_>>()
}