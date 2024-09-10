// BOJ 17495 [Piano]
const INF: u32 = 0x3f3f3f3f;
fn ktoi(key: &String) -> usize {
    let key = key.as_bytes();
    let mut x = match key[0] {
        b'C' => 0, b'D' => 2, b'E' => 4, b'F' => 5, b'G' => 7, b'A' => 9, b'B' => 11, _ => 0
    };
    x += (key[1] - b'0') as usize * 12;
    if key.len() == 3 { x += 1; }
    x
}
pub fn main() { read();
    let init = ktoi(&next()) << 7 | ktoi(&next());
    let n = next::<usize>();

    let mut dp = vec![INF; 1<<14];
    let mut tr = vec![vec![0; 1<<14]; n];
    let mut keys = vec![];
    dp[init] = 0;
    for i in 0..n {
        let k = ktoi(&next()); keys.push(k);
        let mut np = vec![INF; 1<<14];
        for p in 0..1<<14 {
            if dp[p] == INF { continue; }
            let (l, r) = (p >> 7, p & 0x7f);
            // move left hand
            if np[k << 7 | r] > dp[p] + l.abs_diff(k) as u32 {
                np[k << 7 | r] = dp[p] + l.abs_diff(k) as u32;
                tr[i][k << 7 | r] = p as u16;
            }
            // move right hand
            if np[l << 7 | k] > dp[p] + r.abs_diff(k) as u32 {
                np[l << 7 | k] = dp[p] + r.abs_diff(k) as u32;
                tr[i][l << 7 | k] = p as u16;
            }
        }
        dp = np;
    }

    let (mut ans, mut p) = (INF, 0);
    for i in 0..1<<14 {
        if dp[i] < ans { (ans, p) = (dp[i], i); }
    }
    let mut t = vec![];
    for i in (0..n).rev() {
        if p >> 7 == keys[i] { t.push(1); } else { t.push(2); }
        p = tr[i][p] as usize;
    }
    t.reverse();

    println!("{}", ans);
    for i in 0..n { print!("{} ", t[i]); }
    println!();
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