// BOJ 27547 [7Krokods]
fn get(c: &[u32; 4]) -> u32 { // d, k, o, r
    c[0] * c[0] + c[1] * c[1] + c[2] * c[2] + c[3] * c[3] + c[0].min(c[1]/2).min(c[2]/2).min(c[3]) * 7
}
fn go(c: &mut [u32; 4], i: usize, k: u32) -> u32 {
    let mut ret = 0;
    if i == 3 {
        c[i] += k;
        ret = get(c);
        c[i] -= k;
        return ret;
    }
    for x in 0..=k {
        c[i] += x;
        ret = ret.max(go(c, i+1, k-x));
        c[i] -= x;
    }
    ret
}
pub fn main() { read();
    let (_, k) = (next::<u32>(), next::<u32>());
    let mut c = [0; 4];
    for x in next::<String>().chars() {
        match x {
            'd' => c[0] += 1,
            'k' => c[1] += 1,
            'o' => c[2] += 1,
            'r' => c[3] += 1,
            _ => (),
        }
    }
    println!("{}", go(&mut c, 0, k));
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