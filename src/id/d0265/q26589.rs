// BOJ 26589 [Demo Clean Up]
// Supported by GitHub Copilot

pub fn main() { read();
    let s = unsafe { IT.as_mut().unwrap().collect::<Vec<_>>() };
    for v in s.into_iter().map(|t| t.split_whitespace().map(|u| u.parse::<i32>().unwrap()).collect::<Vec<_>>()) {
        if v.is_empty() { break; }
        let x = v[0];
        let mut w = v.into_iter().skip(1).collect::<Vec<_>>();
        w.sort_unstable_by(|a, b| b.cmp(a));

        let mut cnt = 0;
        let mut v = vec![false; w.len()];
        for i in 0..w.len() {
            if v[i] { continue; }
            let mut y = 0;
            for j in i..w.len() {
                if v[j] { continue; }
                if y + w[j] <= x {
                    y += w[j];
                    v[j] = true;
                }
            }
            cnt += 1;
        }
        println!("{}", cnt);
    }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug};
static mut BUF: String = String::new();
static mut IT: Option<std::str::Split<'static, &'static str>> = None;
thread_local! {
    static SI: RefCell<BufReader<StdinLock<'static>>> = RefCell::new(BufReader::new(stdin().lock()));
    static SO: RefCell<BufWriter<StdoutLock<'static>>> = RefCell::new(BufWriter::new(stdout().lock()));
}
fn read() { unsafe {
    BUF.clear();
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split("\n"));
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}