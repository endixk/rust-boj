// BOJ 6569 [Mondriaan's Dream]
const ANS: [u64; 121] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 0, 3, 0, 11, 0, 41, 0, 153, 0, 571, 0, 1, 5, 11, 36, 95, 281, 781, 2245, 6336, 18061, 51205, 0, 8, 0, 95, 0, 1183, 0, 14824, 0, 185921, 0, 1, 13, 41, 281, 1183, 6728, 31529, 167089, 817991, 4213133, 21001799, 0, 21, 0, 781, 0, 31529, 0, 1292697, 0, 53175517, 0, 1, 34, 153, 2245, 14824, 167089, 1292697, 12988816, 108435745, 1031151241, 8940739824, 0, 55, 0, 6336, 0, 817991, 0, 108435745, 0, 14479521761, 0, 1, 89, 571, 18061, 185921, 4213133, 53175517, 1031151241, 14479521761, 258584046368, 3852472573499, 0, 144, 0, 51205, 0, 21001799, 0, 8940739824, 0, 3852472573499, 0];
pub fn main() { read();
    loop {
        let (i, j) = (next::<usize>(), next::<usize>());
        if i + j == 0 { break; }
        println!("{}", ANS[i * 11 + j - 12]);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}