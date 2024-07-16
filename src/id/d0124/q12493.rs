// BOJ 12493 [Space Emergency (Small)]
pub fn main() { read();
    for tc in 1..=next() {
        let (l, t, n, c) = (next::<usize>(), next::<i64>(), next::<usize>(), next::<usize>());
        let a = (0..c).map(|_| next::<i64>()).collect::<Vec<_>>();

        let (mut x, mut s) = (0, 0);
        let mut v = vec![];
        for i in 0..n {
            let k = a[i % c];
            s += 2 * k;
            if x < 0 {
                v.push(k); continue;
            }
            if x + 2 * k > t {
                v.push((2 * k - t + x) / 2);
                x = -1;
            } else {
                x += 2 * k;
            }
        }

        v.sort_unstable_by_key(|&x| -x);
        match l {
            0 => println!("Case #{}: {}", tc, s),
            1 => println!("Case #{}: {}", tc, s - v[0]),
            _ => println!("Case #{}: {}", tc, s - v[0] - v[1]),
        }
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
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