// BOJ 10210 [The Trojan Horse]
fn fill(b: &mut Vec<Vec<bool>>, i: usize, j: usize, h: usize, w: usize) -> usize {
    if b[i][j] { return 0; }
    let mut ret = 0;
    let mut q = vec![(i, j)];
    b[i][j] = true;
    while !q.is_empty() {
        let (i, j) = q.pop().unwrap();
        ret += 1;
        if i > 0 && !b[i-1][j] { b[i-1][j] = true; q.push((i-1, j)); }
        if i+1 < h && !b[i+1][j] { b[i+1][j] = true; q.push((i+1, j)); }
        if j > 0 && !b[i][j-1] { b[i][j-1] = true; q.push((i, j-1)); }
        if j+1 < w && !b[i][j+1] { b[i][j+1] = true; q.push((i, j+1)); }
    }
    ret
}
pub fn main() { read();
    for tc in 0..next() {
        let (h, w, s, n) = (next::<usize>(), next::<usize>(), next::<usize>(), next::<usize>());
        let mut b = vec![vec![false; w]; h];
        for _ in 0..n {
            let (mut i, mut j) = (h-1, 0);
            b[i][j] = true;
            for c in next::<String>().chars() {
                match c {
                    'u' => i -= 1, 'd' => i += 1,
                    'l' => j -= 1, 'r' => j += 1,
                    _ => (),
                }
                b[i][j] = true;
            }
        }

        let mut ans = 0;
        for i in 0..h { for j in 0..w {
            if fill(&mut b, i, j, h, w) >= s { ans += 1; }
        }}
        println!("Data Set {}:\n{}\n", tc+1, ans);
    }
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