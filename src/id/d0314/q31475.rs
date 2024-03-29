// BOJ 31475 [Pigtail Array]
#[inline] fn go(i: usize, j: usize, d: usize) -> (usize, usize) {
    let (ni, nj) = match d {
        1 => (i-1, j), 2 => (i, j+1), 3 => (i+1, j), _ => (i, j-1),
    };
    (ni, nj)
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut a = vec![vec![0; 2002]; 2002];
    for j in 1..=m { a[0][j] = 1; }
    for j in 1..=m { a[n+1][j] = 1; }
    for i in 1..=n { a[i][0] = 1; a[i][m+1] = 1; }

    let (mut ai, mut aj, mut ad, mut bi, mut bj, mut bd, sz) = match next::<char>() {
        'U' => (1, m+1>>1, 3, 1, m+1>>1, 3, n*(m+1>>1)),
        'D' => (n, m+1>>1, 1, n, m+1>>1, 1, n*(m+1>>1)),
        'L' => (n+1>>1, 1, 2, n+1>>1, 1, 2, m*(n+1>>1)),
         _  => (n+1>>1, m, 4, n+1>>1, m, 4, m*(n+1>>1)),
    };
    for i in 1..=sz {
        a[ai][aj] = i; a[bi][bj] = i;
        let (i, j) = go(ai, aj, ad);
        if a[i][j] != 0 {
            ad = (ad + 1) % 4;
            bd = (bd + 3) % 4;
        }
        (ai, aj) = go(ai, aj, ad);
        (bi, bj) = go(bi, bj, bd);
    }

    for i in 1..=n {
        for j in 1..=m {
            print!("{} ", a[i][j]);
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