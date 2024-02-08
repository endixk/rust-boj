// BOJ 6594 [Equation Solver]
#[derive(Clone, Copy, Debug)] struct Element { x: i32, k: i32 }
impl Element {
    fn add(&mut self, rhs: &Element) { self.x += rhs.x; self.k += rhs.k; }
    fn sub(&mut self, rhs: &Element) { self.x -= rhs.x; self.k -= rhs.k; }
    fn mul(&mut self, rhs: &Element) {
        if self.x != 0 && rhs.x != 0 { panic!("non-linear equation"); }
        if self.x == 0 { self.x = rhs.x * self.k; }
        else { self.x *= rhs.k; }
        self.k *= rhs.k;
    }
}
fn parse_number(buf: &[u8], ptr: &mut usize) -> Element {
    let mut k = 0;
    while buf[*ptr].is_ascii_digit() {
        k = k * 10 + (buf[*ptr] - b'0') as i32;
        *ptr += 1;
    }
    Element { x: 0, k }
}
fn parse_factor(buf: &[u8], ptr: &mut usize) -> Element {
    return match buf[*ptr] {
        b'x' => { *ptr += 1; Element { x: 1, k: 0 } },
        b'(' => {
            *ptr += 1;
            let ret = parse_expression(buf, ptr);
            *ptr += 1;
            ret
        },
        _ => parse_number(buf, ptr),
    }
}
fn parse_term(buf: &[u8], ptr: &mut usize) -> Element {
    let mut ret = parse_factor(buf, ptr);
    while buf[*ptr] == b'*' {
        *ptr += 1;
        ret.mul(&parse_factor(buf, ptr));
    }
    ret
}
fn parse_expression(buf: &[u8], ptr: &mut usize) -> Element {
    let mut ret = parse_term(buf, ptr);
    while buf[*ptr] == b'+' || buf[*ptr] == b'-' {
        let op = buf[*ptr];
        *ptr += 1;
        let rhs = parse_term(buf, ptr);
        if op == b'+' { ret.add(&rhs); }
        else { ret.sub(&rhs); }
    }
    ret
}
fn parse_equation(buf: &[u8], ptr: &mut usize) -> (Element, Element) {
    let lhs = parse_expression(buf, ptr);
    *ptr += 1;
    let rhs = parse_expression(buf, ptr);
    (lhs, rhs)
}
pub fn main() { read();
    for tc in 1.. {
        if !peek() { break; }
        println!("Equation #{}", tc);

        let mut buf = next::<String>(); buf.push('@');
        let buf = buf.as_bytes();
        let mut ptr = 0;
        let (lhs, rhs) = parse_equation(buf, &mut ptr);

        let (x, k) = (lhs.x - rhs.x, rhs.k - lhs.k);
        if x == 0 {
            if k == 0 { println!("Infinitely many solutions."); }
            else { println!("No solution."); }
        } else {
            println!("x = {:.6}", k as f64 / x as f64);
        }
        println!();
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, iter::Peekable};
static mut BUF: String = String::new();
static mut IT: Option<Peekable<SplitAsciiWhitespace>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace().peekable());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}
fn peek() -> bool { unsafe { IT.as_mut().unwrap().peek().is_some() } }