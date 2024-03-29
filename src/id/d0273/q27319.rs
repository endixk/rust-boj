// BOJ 27319 [Chino's Latte Art (Hard)]
#[derive(Eq, PartialEq, Clone, Copy, Debug)] struct Point { x: i64, y: i64 }
fn ccw(a: &Point, b: &Point, c: &Point) -> i8 {
    let t = (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x);
    if t > 0 { 1 } else if t < 0 { -1 } else { 0 }
}

fn sym(a: &Point, b: &Point, p1: &Point, p2: &Point) -> bool {
    (p1.x - a.x).pow(2) + (p1.y - a.y).pow(2) == (p2.x - a.x).pow(2) + (p2.y - a.y).pow(2) &&
    (p1.x - b.x).pow(2) + (p1.y - b.y).pow(2) == (p2.x - b.x).pow(2) + (p2.y - b.y).pow(2)
}

pub fn main() { read();
    let n = next::<usize>();
    if n & 1 == 1 { println!("0"); return; }

    let p = (0..n).map(|_| Point { x: next(), y: next() }).collect::<Vec<_>>();
    let mut c = vec![];
    c.push(ccw(&p[n-1], &p[0], &p[1]));
    (1..n-1).for_each(|i| c.push(ccw(&p[i-1], &p[i], &p[i+1])));
    c.push(ccw(&p[n-2], &p[n-1], &p[0]));


    let k = match c.iter().position(|&x| x == 1) { Some(k) => { k }, _ => { println!("0"); return; } };
    if c[k+1..].iter().any(|&x| x == 1) { println!("0"); return; }
    let r = (k + n/2) % n;
    let (mut i, mut j) = (if k == 0 { n-1 } else { k-1 }, if k == n-1 { 0 } else { k+1 });
    for _ in 1..n>>1 {
        if !sym(&p[k], &p[r], &p[i], &p[j]) { println!("0"); return; }
        (i, j) = (if i == 0 { n-1 } else { i-1 }, if j == n-1 { 0 } else { j+1 });
    }
    println!("1");
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