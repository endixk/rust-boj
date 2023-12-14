// BOJ 30701 [Return of the Kusoge]
// Supported by GitHub Copilot

pub fn main() { read();
    const INF: u128 = u64::MAX as u128;
    let (n, mut d) = (next::<usize>(), next::<u128>());
    let (mut f, mut e) = (vec![], vec![]);
    for _ in 0..n {
        match next::<u8>() {
            1 => f.push(next::<u128>()),
            _ => e.push(next::<u128>()),
        }
    }
    f.sort_unstable();
    e.sort_unstable();

    let (mut i, mut j) = (0, 0);
    while i < f.len() {
        if f[i] >= d && j == e.len() { break; }
        if f[i] < d {
            d += f[i]; i += 1;
        } else {
            d *= e[j]; j += 1;
        }
        if d > INF { break; }
    }

    println!("{}", if d > INF { n } else { i + e.len() });
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