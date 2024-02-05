// BOJ 6587 [France '98]
fn gcd(a: i128, b: i128) -> i128 { if b == 0 { a } else { gcd(b, a % b) } }
#[derive(Debug, Clone, Copy)] struct Frac { n: i128, d: i128 }
impl Frac {
    fn new(n: i128, d: i128) -> Self { let g = gcd(n.abs(), d.abs()); Self { n: n / g, d: d / g } }
    fn rdc(&mut self) { let g = gcd(self.n.abs(), self.d.abs()); self.n /= g; self.d /= g; }
    fn add(&mut self, rhs: Self) { self.n = self.n * rhs.d + rhs.n * self.d; self.d *= rhs.d; self.rdc(); }
    fn mul(&mut self, rhs: Self) { self.n *= rhs.n; self.d *= rhs.d; self.rdc(); }
}
fn comb(a: &[Frac; 16], b: &[Frac; 16], probs: &Vec<Vec<i128>>) -> [Frac; 16] {
    let mut res = [Frac::new(0, 1); 16];
    for i in 0..16 {
        for j in 0..16 {
            let mut p = Frac::new(probs[i][j], 100);
            let mut q = Frac::new(probs[j][i], 100);
            p.mul(a[i]); p.mul(b[j]);
            q.mul(a[i]); q.mul(b[j]);
            res[i].add(p); res[j].add(q);
        }
    }
    res
}
pub fn main() { read();
    let names = (0..16).map(|_| next::<String>()).collect::<Vec<_>>();
    let probs = (0..16).map(|_| (0..16).map(|_| next::<i128>()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut r16 = vec![[Frac::new(0, 1); 16]; 16];
    for i in 0..16 { r16[i][i] = Frac::new(1, 1); }
    let r8 = (0..8).map(|i| comb(&r16[i*2], &r16[i*2+1], &probs)).collect::<Vec<_>>();
    let r4 = (0..4).map(|i| comb(&r8[i*2], &r8[i*2+1], &probs)).collect::<Vec<_>>();
    let r2 = (0..2).map(|i| comb(&r4[i*2], &r4[i*2+1], &probs)).collect::<Vec<_>>();
    let r1 = comb(&r2[0], &r2[1], &probs);

    for i in 0..16 {
        println!("{:10} p={:.2}%", names[i], r1[i].n as f64 / r1[i].d as f64 * 100.0);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}