// BOJ 24507 [blobfacepalm]
// backtracking solution for smaller N
fn langford_small(seq: &mut Vec<usize>, n: usize, i: usize) -> bool {
    if i == 0 { return true; }
    for j in 0..(n<<1) - i {
        if seq[j] == n && seq[j + i] == n {
            seq[j] = i - 1;
            seq[j + i] = i - 1;
            if langford_small(seq, n, i - 1) {
                return true;
            }
            seq[j] = n;
            seq[j + i] = n;
        }
    }
    false
}
// constructive solution for larger N, reference: https://doi.org/10.2307/3610650
// case n = 4m
fn langford_4m(m: usize) {
    let (m2, m4) = (m<<1, m<<2);
    (m2..=m4-4).rev().step_by(2).for_each(|i| print!("{} ", i));
    print!("{} ", m4-2);
    (1..=m2-3).rev().step_by(2).for_each(|i| print!("{} ", i));
    print!("{} ", m4-1);
    (1..=m2-3).step_by(2).for_each(|i| print!("{} ", i));
    (m2..=m4-4).step_by(2).for_each(|i| print!("{} ", i));
    print!("{} ", m4);
    (m2+1..=m4-3).rev().step_by(2).for_each(|i| print!("{} ", i));
    print!("{} ", m4-2);
    (2..=m2-2).rev().step_by(2).for_each(|i| print!("{} ", i));
    print!("{} {} ", m2-1, m4-1);
    (2..=m2-2).step_by(2).for_each(|i| print!("{} ", i));
    (m2+1..=m4-3).step_by(2).for_each(|i| print!("{} ", i));
    println!("{} {} 0 0", m2-1, m4);
}
// case n = 4m - 1
fn langford_4m1(m: usize) {
    let (m2, m4) = (m<<1, m<<2);
    (m2..=m4-4).rev().step_by(2).for_each(|i| print!("{} ", i));
    print!("{} ", m4-2);
    (1..=m2-3).rev().step_by(2).for_each(|i| print!("{} ", i));
    print!("{} ", m4-1);
    (1..=m2-3).step_by(2).for_each(|i| print!("{} ", i));
    (m2..=m4-4).step_by(2).for_each(|i| print!("{} ", i));
    print!("{} ", m2-1);
    (m2+1..=m4-3).rev().step_by(2).for_each(|i| print!("{} ", i));
    print!("{} ", m4-2);
    (2..=m2-2).rev().step_by(2).for_each(|i| print!("{} ", i));
    print!("{} {} ", m2-1, m4-1);
    (2..=m2-2).step_by(2).for_each(|i| print!("{} ", i));
    (m2+1..=m4-3).step_by(2).for_each(|i| print!("{} ", i));
    println!("0 0");
}

pub fn main() { read();
    let n = next::<usize>();
    if n % 4 == 2 || n % 4 == 3 {
        println!("No");
        return;
    }
    println!("Yes");
    if n < 10 {
        let mut seq = vec![n; n<<1];
        langford_small(&mut seq, n, n);
        println!("{}", seq.iter().map(|&x| x.to_string()).collect::<Vec<_>>().join(" "));
    } else if n % 4 == 0 {
        langford_4m1(n>>2);
    } else {
        langford_4m((n-1)>>2);
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