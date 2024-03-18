// BOJ 31479 [Pigtail Squared]
fn gcd(a: u32, b: u32) -> u32 { if b == 0 { a } else { gcd(b, a % b) }}
#[derive(Clone, Copy, Debug, Default, PartialEq)] struct Term { sgn: bool, nom: u32, den: u32, pow: u32 }
impl Term {
    fn reduce(&mut self) {
        let g = gcd(self.nom, self.den);
        (self.nom, self.den) = (self.nom / g, self.den / g);
    }
    fn diff(&mut self) {
        if self.pow == 0 { *self = Term::default(); }
        else {
            self.nom *= self.pow;
            self.pow -= 1;
            self.reduce();
        }
    }
    fn is_default(&self) -> bool { self.nom == 0 }
}
fn parse(s: &[u8]) -> Vec<Term> {
    let mut v = vec![];
    let mut i = 0;
    while i < s.len() {
        let mut t = Term::default();
        if s[i] == b'-' { t.sgn = true; i += 1; }
        else if s[i] == b'+' { i += 1; }
        if s[i] == b'C' { break; }
        if s[i].is_ascii_digit() {
            while i < s.len() && s[i].is_ascii_digit() {
                t.nom = t.nom * 10 + (s[i] - b'0') as u32;
                i += 1;
            }
        } else { t.nom = 1; }
        if i < s.len() && s[i] == b'/' {
            i += 1;
            while i < s.len() && s[i].is_ascii_digit() {
                t.den = t.den * 10 + (s[i] - b'0') as u32;
                i += 1;
            }
        } else { t.den = 1; }
        if i < s.len() && s[i] == b'x' {
            i += 1;
            if i < s.len() && s[i] == b'^' {
                i += 1;
                while i < s.len() && s[i].is_ascii_digit() {
                    t.pow = t.pow * 10 + (s[i] - b'0') as u32;
                    i += 1;
                }
            } else { t.pow = 1; }
        } else { t.pow = 0; }
        v.push(t);
    }
    v
}
pub fn main() { read();
    for _ in 0..next() {
        let mut a = parse(&next::<String>().into_bytes());
        for t in a.iter_mut() { t.diff(); t.diff(); }
        let a = a.into_iter().filter(|t| !t.is_default()).collect::<Vec<_>>();
        let b = parse(&next::<String>().into_bytes()).into_iter().filter(|t| !t.is_default()).collect::<Vec<_>>();
        println!("{}", if a == b { "Yes" } else { "No" });
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