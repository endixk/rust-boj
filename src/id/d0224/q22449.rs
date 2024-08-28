// BOJ 22449 [Almost Same Substring]
fn z(s: &String) -> Vec<usize> {
    let (s, n) = (s.as_bytes(), s.len());
    let mut z = vec![0; n];
    z[0] = n;

    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i > r {
            l = i; r = i;
            while r < n && s[r-l] == s[r] { r += 1; }
            z[i] = r-l; r -= 1;
        } else {
            let k = i-l;
            if z[k] < r-i+1 { z[i] = z[k]; }
            else {
                l = i;
                while r < n && s[r-l] == s[r] { r += 1; }
                z[i] = r-l; r -= 1;
            }
        }
    }

    z
}
pub fn main() { read();
    let (s, t) = (next::<String>(), next::<String>());
    let mut x = t.clone(); x.extend(s.chars());
    let mut y = t.chars().rev().collect::<String>(); y.extend(s.chars().rev());
    let (zx, zy) = (z(&x), z(&y));

    let (n, m) = (s.len(), t.len());
    println!("{}", (0..=n-m).filter(|&i| zx[m+i] + zy[n-i] == m-1).count());
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