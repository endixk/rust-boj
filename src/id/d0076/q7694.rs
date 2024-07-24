// BOJ 7694 [Triangle]
const INF: i32 = 0x3f3f3f3f;
fn isect(x1: i32, y1: i32, x2: i32, y2: i32, y: i32) -> Option<(i32, i32)> {
    if y1 > y2 { return isect(x2, y2, x1, y1, y); }
    if y < y1 || y > y2 { return None; }
    if y1 == y2 { return None; }
    let (n, d) = ((y - y1) * (x2 - x1), y2 - y1);
    return if n % d == 0 {
        (x1 + n / d - 1, x1 + n / d + 1).into()
    } else if n > 0 {
        (x1 + n / d, x1 + n / d + 1).into()
    } else {
        (x1 + n / d - 1, x1 + n / d).into()
    };
}
pub fn main() { read();
    loop {
        let (x1, y1) = (next::<i32>(), next::<i32>());
        let (x2, y2) = (next::<i32>(), next::<i32>());
        let (x3, y3) = (next::<i32>(), next::<i32>());
        if x1 | y1 | x2 | y2 | x3 | y3 == 0 { break; }

        let (ymin, ymax) = (y1.min(y2).min(y3), y1.max(y2).max(y3));
        let mut ans = 0;
        for y in ymin+1..ymax {
            let xa = isect(x1, y1, x2, y2, y);
            let xb = isect(x2, y2, x3, y3, y);
            let xc = isect(x3, y3, x1, y1, y);
            let (xal, xar) = xa.unwrap_or((-INF, INF));
            let (xbl, xbr) = xb.unwrap_or((-INF, INF));
            let (xcl, xcr) = xc.unwrap_or((-INF, INF));
            ans += xal.max(xbl).max(xcl) - xar.min(xbr).min(xcr) + 1;
        }
        println!("{}", ans);
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
static mut BUF: String = String::new();
static mut IT: Option<SplitAsciiWhitespace> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut *addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}