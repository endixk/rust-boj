// BOJ 28978 [Crosswords]
// Supported by GitHub Copilot

fn place_h2(v1: &[u8], y1: usize, v2: &[u8], y2: usize, h2: &[u8], d: usize) -> usize {
    if d >= h2.len() { return 0; }
    let mut ret = 0;
    for i in 0..v1.len() {
        if i == y1 { continue; }
        let off = if i < y1 { y1 - i } else { i - y1 };
        if i < y1 && y2 < off { continue; }
        if i > y1 && y2 + off >= v2.len() { continue; }

        let (c1, c2) = (v1[i], v2[if i < y1 { y2 - off } else { y2 + off }]);
        for j in 0..h2.len()-d {
            if h2[j] == c1 && h2[j+d] == c2 {
                ret += 1;
            }
        }
    }
    ret
}
fn place_v2(h1: &[u8], x: usize, v1: &[u8], y: usize, v2: &[u8], h2: &[u8]) -> usize {
    let mut ret = 0;
    for i in 0..h1.len() {
        if i == x { continue; }
        for j in 0..v2.len() {
            if h1[i] == v2[j] {
                if i < x {
                    ret += place_h2(v2, j, v1, y, h2, x - i);
                } else {
                    ret += place_h2(v1, y, v2, j, h2, i - x);
                }
            }
        }
    }
    ret
}
fn place_v1(h1: &[u8], h2: &[u8], v1: &[u8], v2: &[u8]) -> usize {
    let mut ret = 0;
    for i in 0..h1.len() {
        for j in 0..v1.len() {
            if h1[i] == v1[j] {
                ret += place_v2(h1, i, v1, j, v2, h2);
            }
        }
    }
    ret
}
pub fn main() { read();
    let (a, b, c, d) = (next::<String>(), next::<String>(), next::<String>(), next::<String>());
    let (a, b, c, d) = (a.as_bytes(), b.as_bytes(), c.as_bytes(), d.as_bytes());
    let mut ans = 0;
    ans += place_v1(a, b, c, d);
    ans += place_v1(a, c, b, d);
    ans += place_v1(a, d, b, c);
    println!("{}", ans * 2);
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