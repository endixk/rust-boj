// BOJ 30973 [Shoot! Take a Panorama]

#[inline] fn get(x: i64, y: i64, c1: i64, c2: i64, c3: i64, c4: i64) -> i64 {
    x * x * c1 + y * y * c1 - 2 * x * c2 - 2 * y * c3 + c4
}
pub fn main() { read();
    let n = next::<i64>();
    let (x1, y1, x2, y2) = (next::<i64>(), next::<i64>(), next::<i64>(), next::<i64>());

    let (mut c1, mut c2, mut c3, mut c4) = (0, 0, 0, 0);
    for _ in 0..n {
        let (x, y, l) = (next::<i64>(), next::<i64>(), next::<i64>());
        c1 += l; c2 += l * x; c3 += l * y;
        c4 += l * x * x + l * y * y;
    }
    if c1 == 0 { println!("0"); return; }

    let mut ans = i64::MAX;
    ans = ans.min(get(c2/c1,   y1-1,    c1, c2, c3, c4));
    ans = ans.min(get(c2/c1+1, y1-1,    c1, c2, c3, c4));
    ans = ans.min(get(c2/c1-1, y1-1,    c1, c2, c3, c4));
    ans = ans.min(get(c2/c1,   y2+1,    c1, c2, c3, c4));
    ans = ans.min(get(c2/c1+1, y2+1,    c1, c2, c3, c4));
    ans = ans.min(get(c2/c1-1, y2+1,    c1, c2, c3, c4));
    ans = ans.min(get(x1-1,    c3/c1,   c1, c2, c3, c4));
    ans = ans.min(get(x1-1,    c3/c1+1, c1, c2, c3, c4));
    ans = ans.min(get(x1-1,    c3/c1-1, c1, c2, c3, c4));
    ans = ans.min(get(x2+1,    c3/c1,   c1, c2, c3, c4));
    ans = ans.min(get(x2+1,    c3/c1+1, c1, c2, c3, c4));
    ans = ans.min(get(x2+1,    c3/c1-1, c1, c2, c3, c4));
    println!("{}", ans);
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