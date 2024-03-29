// BOJ 6572 [Phylogenetic Trees Inherited]
pub fn main() { read();
    loop {
        let (n, l) = (next::<usize>(), next::<usize>());
        if n + l == 0 { break; }

        let n = n.next_power_of_two();
        let mut c = vec![vec![0; n<<1]; l];
        for j in 0..n {
            let s = next::<String>().into_bytes();
            for i in 0..l {
                c[i][n|j] |= 1 << (s[i] - b'A');
            }
        }

        // let ord = "ARNDCQEGHILKMFPSTWYV".chars().map(|c| (c as u8 - b'A') as usize).collect::<Vec<_>>();
        let mut lca = String::new();
        let mut ans = 0;
        for i in 0..l {
            for j in (1..n).rev() {
                c[i][j] = c[i][j<<1] & c[i][j<<1|1];
                if c[i][j] == 0 {
                    ans += 1;
                    c[i][j] = c[i][j<<1] | c[i][j<<1|1];
                }
            }
            let mut x = 0;
            for j in (0..26).rev() { if c[i][1] & (1 << j) != 0 { x = j; break; } }
            lca.push((x as u8 + b'A') as char);
        }

        println!("{} {}", lca, ans);
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