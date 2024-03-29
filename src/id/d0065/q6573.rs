// BOJ 6573 [Hike on a Graph]
pub fn main() { read();
    loop {
        let n = next::<usize>();
        if n == 0 { break; }
        let (a, b, c) = (next::<usize>()-1, next::<usize>()-1, next::<usize>()-1);

        let mut adj = vec![vec![0u8; n]; n];
        for i in 0..n { for j in 0..n {
            adj[i][j] = next::<char>() as u8;
        }}

        let mut d = 0;
        let mut flag = false;
        let mut v = vec![false; 1<<18];
        v[a<<12|b<<6|c] = true;
        let mut q = vec![a<<12|b<<6|c];
        while !q.is_empty() {
            let mut nq = vec![];
            for x in q {
                let (a, b, c) = (x>>12, x>>6&0x3f, x&0x3f);
                if a == b && b == c { flag = true; break; }
                let (ca, cb, cc) = (adj[b][c], adj[a][c], adj[a][b]);
                for i in 0..n {
                    if adj[a][i] == ca && !v[i<<12|b<<6|c] {
                        v[i<<12|b<<6|c] = true;
                        nq.push(i<<12|b<<6|c);
                    }
                    if adj[b][i] == cb && !v[a<<12|i<<6|c] {
                        v[a<<12|i<<6|c] = true;
                        nq.push(a<<12|i<<6|c);
                    }
                    if adj[c][i] == cc && !v[a<<12|b<<6|i] {
                        v[a<<12|b<<6|i] = true;
                        nq.push(a<<12|b<<6|i);
                    }
                }
            }
            if flag { break; }
            d += 1; q = nq;
        }
        if flag { println!("{}", d); }
        else { println!("impossible"); }
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