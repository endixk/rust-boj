// BOJ 28070 [Letters]
#[inline] fn conv(s: String) -> usize {
    let (y, m) = (s[0..4].parse::<usize>().unwrap() - 2000, s[5..7].parse::<usize>().unwrap() - 1);
    y * 12 + m
}
const MAX: usize = 96969;
pub fn main() { read();
    let n = next::<usize>();
    let mut imos = vec![0; MAX];
    for _ in 0..n {
        let (s, e) = (conv(next()), conv(next()));
        imos[s] += 1;
        imos[e+1] -= 1;
    }

    let (mut max, mut ans, mut cnt) = (0, 0, 0);
    for i in 0..MAX {
        cnt += imos[i];
        if max < cnt {
            max = cnt;
            ans = i;
        }
    }

    println!("{}-{:02}", ans / 12 + 2000, ans % 12 + 1);
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