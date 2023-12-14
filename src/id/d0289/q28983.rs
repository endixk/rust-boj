// BOJ 28983 [Secret Rooms]
// Supported by GitHub Copilot

pub fn main() { read();
    let n = next::<usize>();
    let mut p = vec![0; n];
    let mut ind = vec![0; n];
    for i in 0..n {
        let x = next::<usize>() - 1;
        p[i] = x;
        ind[x] += 1;
    }
    if ind.iter().filter(|&&x| x == 0).count() != 1 {
        println!("-1 -1"); return;
    }

    let mut k = ind.iter().position(|&x| x == 0).unwrap();
    let mut vis = vec![false; n];
    let mut path = vec![k];
    while !vis[k] {
        vis[k] = true;
        k = p[k];
        path.push(k);
    }

    if vis.iter().any(|&x| !x) {
        println!("-1 -1");
    } else {
        path.pop();
        println!("{} {}", path.last().unwrap() + 1, path[0] + 1);
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}