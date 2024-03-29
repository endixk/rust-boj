// BOJ 28965 [Magical Spheres]
// Supported by GitHub Copilot

extern "C" { fn srand(seed: u64) -> u32; fn rand() -> u32; }
fn csrand(n: u64) { unsafe { srand(n); } }
fn crand() -> u32 { unsafe { rand() } }
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());

    csrand((n + m) as u64);
    let mut sum = 0;
    let mut edges = vec![];
    for _ in 0..m {
        let (u, v, w) = (next::<usize>(), next::<usize>(), next::<i64>());
        sum += w; edges.push((w, u, v));
    }

    loop {
        let mut color = vec![0; n+1];
        for i in 1..=n { color[i] = crand() % 2; }
        let mut r = 0;
        for &(w, u, v) in &edges {
            if color[u] == color[v] { r += w; }
        }
        if r <= sum/2 {
            for i in 1..=n { print!("{} ", color[i]); }
            println!(); break;
        }
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