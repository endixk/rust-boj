// BOJ 6585 [Team Queue]
use std::collections::VecDeque;
pub fn main() { read();
    for tc in 1.. {
        let t = next::<usize>();
        if t == 0 { break; }

        let mut v = vec![0; 1000000];
        for i in 0..t {
            for _ in 0..next() { v[next::<usize>()] = i; }
        }

        let mut tq = vec![VecDeque::new(); t];
        let mut q = VecDeque::new();
        println!("Scenario #{}", tc);
        loop {
            let qry = next::<String>();
            if qry == "STOP" { break; }
            if qry == "ENQUEUE" {
                let x = next::<usize>();
                let i = v[x];
                if tq[i].is_empty() { q.push_back(i); }
                tq[i].push_back(x);
            } else {
                let i = *q.front().unwrap();
                println!("{}", tq[i].pop_front().unwrap());
                if tq[i].is_empty() { q.pop_front(); }
            }
        }
        println!();
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