// BOJ 31633 [Cutting-Edge Eggplant Farming]
const DX: [usize; 4] = [0, 1, 0, !0];
const DY: [usize; 4] = [1, 0, !0, 0];
fn fill(locs: &mut Vec<u32>, hmax: &mut i32, h: &mut Vec<i32>,
        hori: i32, i: usize, j: usize, n: usize, m: usize) {
    locs.push((i<<10|j) as u32); h[i<<10|j] -= 1;
    for (&dx, &dy) in DX.iter().zip(DY.iter()) {
        let (ni, nj) = (i.wrapping_add(dx), j.wrapping_add(dy));
        if ni >= n || nj >= m { continue; }
        if h[ni<<10|nj] == hori {
            fill(locs, hmax, h, hori, ni, nj, n, m);
        } else {
            *hmax = (*hmax).max(h[ni<<10|nj]);
        }
    }
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut h = vec![0; 1<<20];
    let mut v = Vec::with_capacity(1<<20);
    for i in 0..n { for j in 0..m {
        h[i<<10|j] = -next::<i32>();
        v.push((i<<10|j) as u32);
    }}
    v.sort_unstable_by_key(|&x| -h[x as usize]);

    for x in v {
        let (i, j) = (x as usize >> 10, x as usize & 1023);
        if h[i<<10|j] < 0 {
            let mut locs = vec![];
            let mut hmax = -1;
            let hori = h[i<<10|j];
            fill(&mut locs, &mut hmax, &mut h, hori, i, j, n, m);
            for x in locs { h[x as usize] = hmax + 1; }
        }
    }

    for i in 0..n { for j in 0..m {
        print!("{} ", h[i<<10|j]);
    } println!(); }
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
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