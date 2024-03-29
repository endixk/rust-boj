// BOJ 31420 [String-Graph Matching]
fn id(s: &[char]) -> usize { (s[0] as usize - 'a' as usize) * 26 + s[1] as usize - 'a' as usize }
pub fn main() { read();
    let (n, s) = (next::<usize>(), next::<String>().chars().collect::<Vec<_>>());
    let mut g = vec![false; 676];
    let d = next::<usize>();
    for _ in 0..d {
        g[id(next::<String>().chars().collect::<Vec<_>>().as_slice())] = true;
    }
    let mut p = vec![false; n+1];
    for i in 1..n { p[i] = g[id(&s[i-1..=i])]; }

    let mut ans = 0;
    let mut c = vec![0; 676];
    let (mut k, mut j, mut x) = (d, 0, 0);
    for i in 1..n {
        if !p[i] { k = d; c.fill(0); continue; }
        j = i.max(j);
        while k > 0 && p[j] {
            let x = id(&s[j-1..=j]);
            if c[x] == 0 { k -= 1; }
            c[x] += 1;
            j += 1;
        }
        if x < j {
            x = j;
            while p[x] { x += 1; }
        }
        if k == 0 { ans += x - j + 1; }
        let x = id(&s[i-1..=i]);
        if c[x] == 1 { k += 1; }
        c[x] -= 1;
    }
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut *std::ptr::addr_of_mut!(BUF)).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}