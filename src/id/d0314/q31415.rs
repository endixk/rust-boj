// BOJ 31415 [UFO Invasion]
const SZ: usize = 100_001;
fn update(a: &mut Vec<i32>, d: usize, mut i: usize, q: &Vec<(usize, usize)>) {
    let mut p = vec![0; SZ/d+1];
    for &(l, r) in q {
        p[l] += 1;
        if r * d + i < SZ { p[r] -= 1; }
    }
    let (mut c, mut j) = (0, 0);
    while i < SZ {
        c += p[j]; a[i] += c; j += 1; i += d;
    }
}
pub fn main() { read();
    let (n, q, t) = (next::<usize>(), next::<usize>(), next::<usize>());
    let (mut x, mut y) = (vec![], vec![]);
    let (mut xp, mut yp) = (vec![0; SZ], vec![0; SZ]);
    for _ in 0..n {
        let (ix, iy) = (next::<usize>(), next::<usize>());
        let (dx, dy) = (next::<i32>(), next::<i32>());
        if dx == 0 {
            xp[ix] += 1;
        } else if dx < 0 {
            let d = -dx as usize;
            let i = ix % d;
            let t = if ix < d * t - d { i } else { ix + d - d * t };
            x.push((d, i, (t - i) / d, (ix + d - i) / d));
        } else {
            let d = dx as usize;
            let i = ix % d;
            x.push((d, i, (ix - i) / d, (ix + d * t - i) / d));
        }
        if dy == 0 {
            yp[iy] += 1;
        } else if dy < 0 {
            let d = -dy as usize;
            let i = iy % d;
            let t = if iy < d * t - d { i } else { iy + d - d * t };
            y.push((d, i, (t - i) / d, (iy + d - i) / d));
        } else {
            let d = dy as usize;
            let i = iy % d;
            y.push((d, i, (iy - i) / d, (iy + d * t - i) / d));
        }
    }
    x.sort_unstable(); y.sort_unstable();

    if !x.is_empty() {
        let mut v = vec![];
        let (mut cd, mut ci) = (x[0].0, x[0].1);
        for (d, i, lx, rx) in x {
            if d != cd || i != ci {
                update(&mut xp, cd, ci, &v);
                v.clear();
                cd = d;
                ci = i;
            }
            v.push((lx, rx));
        }
        update(&mut xp, cd, ci, &v);
    }

    if !y.is_empty() {
        let mut v = vec![];
        let (mut cd, mut ci) = (y[0].0, y[0].1);
        for (d, i, ly, ry) in y {
            if d != cd || i != ci {
                update(&mut yp, cd, ci, &v);
                v.clear();
                cd = d;
                ci = i;
            }
            v.push((ly, ry));
        }
        update(&mut yp, cd, ci, &v);
    }

    for _ in 0..q {
        match next::<u8>() {
            1 => println!("{}", yp[next::<usize>()]),
            _ => println!("{}", xp[next::<usize>()]),
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