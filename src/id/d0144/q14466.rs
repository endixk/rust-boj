// BOJ 14466 [Why Did the Cow Cross the Road III (Silver)]
const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];
pub fn main() { read();
    let (n, k, r) = (next::<usize>(), next::<usize>(), next::<usize>());

    let mut a = [true; 1<<16];
    for _ in 0..r {
        let (r1, c1) = (next::<usize>() - 1, next::<usize>() - 1);
        let (r2, c2) = (next::<usize>() - 1, next::<usize>() - 1);
        if r1 == r2 {
            if c1 < c2 { a[r1<<9|c1<<2|1] = false; a[r2<<9|c2<<2|3] = false; }
            else { a[r1<<9|c1<<2|3] = false; a[r2<<9|c2<<2|1] = false; }
        } else {
            if r1 < r2 { a[r1<<9|c1<<2|0] = false; a[r2<<9|c2<<2|2] = false; }
            else { a[r1<<9|c1<<2|2] = false; a[r2<<9|c2<<2|0] = false; }
        }
    }

    let mut x = [false; 1<<14];
    for _ in 0..k { let (r, c) = (next::<usize>() - 1, next::<usize>() - 1); x[r<<7|c] = true; }

    let mut ans = k * (k - 1) / 2;
    let mut v = [false; 1<<14];
    for i in 0..n { for j in 0..n {
        if v[i<<7|j] { continue; }
        let mut c = 0;
        let mut q = vec![i<<7|j]; v[i<<7|j] = true;
        while let Some(k) = q.pop() {
            let (i, j) = (k>>7, k&127);
            if x[i<<7|j] { c += 1; }
            for d in 0..4 {
                if !a[i<<9|j<<2|d] { continue; }
                let (ni, nj) = (i as i32 + DX[d], j as i32 + DY[d]);
                if ni < 0 || ni >= n as i32 || nj < 0 || nj >= n as i32 { continue; }
                let (ni, nj) = (ni as usize, nj as usize);
                if v[ni<<7|nj] { continue; }
                v[ni<<7|nj] = true; q.push(ni<<7|nj);
            }
        }
        ans -= c * (c - 1) / 2;
    }}
    println!("{}", ans);
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