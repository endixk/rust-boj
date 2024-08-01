// BOJ 10852 [Carpets]
fn left(r: &Vec<(usize, usize)>) -> usize {
    let mut c = r.iter().map(|&(j, x)| j+x).collect::<Vec<usize>>();
    c.push(0); c.sort();
    for j in c {
        if r.iter().any(|&(k, _)| k == j) { continue; }
        return j;
    }
    0
}
fn locate(r: &Vec<Vec<(usize, usize)>>, w: usize) -> (usize, usize) {
    let mut i = 0;
    while r[i].iter().map(|&(_, x)| x).sum::<usize>() == w { i += 1; }
    (i, left(&r[i]))
}
fn place(r: &Vec<Vec<(usize, usize)>>, i: usize, j: usize, h: usize, w: usize, x: usize, y: usize) -> bool {
    if i+x > h || j+y > w { return false; }
    r[i].iter().all(|&(k, z)| k+z <= j || j+y <= k)
}
fn go(f: &mut Vec<bool>, r: &mut Vec<Vec<(usize, usize)>>,
      c: &Vec<(usize, usize)>, n: usize, h: usize, w: usize, x: usize) {
    if x == h * w { println!("yes"); std::process::exit(0); }

    let (i, j) = locate(r, w);
    for k in 0..n {
        if f[k] { continue; }
        if place(r, i, j, h, w, c[k].0, c[k].1) {
            f[k] = true;
            for i in i..i+c[k].0 { r[i].push((j, c[k].1)); }
            go(f, r, c, n, h, w, x+c[k].0*c[k].1);
            for i in i..i+c[k].0 { r[i].pop(); }
            f[k] = false;
        }
        if place(r, i, j, h, w, c[k].1, c[k].0) {
            f[k] = true;
            for i in i..i+c[k].1 { r[i].push((j, c[k].0)); }
            go(f, r, c, n, h, w, x+c[k].0*c[k].1);
            for i in i..i+c[k].1 { r[i].pop(); }
            f[k] = false;
        }
    }
}
pub fn main() { read();
    let (h, w, k) = (next::<usize>(), next::<usize>(), next::<usize>());
    let mut n = 0;
    let mut c = vec![];
    for _ in 0..k {
        let (x, i, j) = (next::<usize>(), next::<usize>(), next::<usize>());
        n += x;
        for _ in 0..x { c.push((i, j)); }
    }
    let mut f = vec![false; n];
    let mut r = vec![vec![]; h];
    go(&mut f, &mut r, &c, n, h, w, 0);
    println!("no");
}

#[allow(unused_macros)] macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_macros)] macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
#[allow(unused_imports)] use println;
#[allow(unused_imports)] use print;
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