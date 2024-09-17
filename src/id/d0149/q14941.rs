// BOJ 14941 [Curiosity]
fn sieve(n: usize) -> Vec<usize> {
    let mut p = vec![true; n+1];
    p[0] = false; p[1] = false;
    for i in 2..=n {
        if !p[i] { continue; }
        for j in (i*i..=n).step_by(i) {
            p[j] = false;
        }
    }
    p.iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i).collect()
}
const MAX: usize = 100_000;
pub fn main() { read();
    let p = sieve(MAX+10);
    let (mut v, mut w, mut j) = (vec![1; MAX+1], vec![1; MAX+1], 0);
    for i in 2..=MAX {
        v[i] = j + 1;
        if p[j] == i { j += 1; }
        w[i] = j;
    }

    let (mut pod, mut pev) = (vec![0], vec![0]);
    for i in 0..p.len() {
        if i & 1 == 0 { pod.push(*pod.last().unwrap() + p[i])}
        else { pev.push(*pev.last().unwrap() + p[i]); }
    }

    for _ in 0..next() {
        let (p, q) = (next::<usize>(), next::<usize>());
        if q == 1 { println!("0"); continue; }
        let (a, b) = (v[if p == 1 { 2 } else { p }], w[q]);
        let (ox, oy) = (if a & 1 == 1 { a } else { a + 1 } >> 1, if b & 1 == 1 { b } else { b - 1 } >> 1);
        let (ex, ey) = (if a & 1 == 0 { a } else { a + 1 } >> 1, if b & 1 == 0 { b } else { b - 1 } >> 1);
        if a & 1 == 1 { println!{"{}", pod[oy+1] * 3 + pev[ex-1] - pod[ox] * 3 - pev[ey] } }
        else { println!{"{}", pev[ey] * 3 + pod[ox] - pev[ex-1] * 3 - pod[oy+1]} }
    }
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
#[allow(unused_imports)] use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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