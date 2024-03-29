// BOJ 27314 [Lucky Hanbyeol]
const DX: [i32; 4] = [0, 0, -1, 1];
const DY: [i32; 4] = [-1, 1, 0, 0];
const MAX: usize = 1<<18;
const QSZ: usize = 1<<10;
const QMASK: usize = QSZ-1;
fn bfs(map: &[u8; MAX], q: &mut [usize; QSZ],
       i: usize, j: usize, b: u32, c: usize, n: usize, m: usize) -> usize {
    let mut v = [false; MAX];
    let (mut it, mut ptr) = (1, 0);
    let mut ret = 0;
    v[i<<b|j] = true; q[0] = i<<b|j;
    let mut flag = false;
    while it != ptr {
        for _ in 0..(it|QSZ)-ptr&QMASK {
            let k = q[ptr]; ptr = ptr+1&QMASK;
            let (x, y) = (k>>b, k&c);
            if map[x<<b|y] == b'H' { flag = true; }
            if map[x<<b|y] == b'P' { ret += 1; }
            if flag { continue; }
            for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                let (i, j) = (x as i32 + dx, y as i32 + dy);
                if i < 0 || i >= n as i32 || j < 0 || j >= m as i32 { continue; }
                let (i, j) = (i as usize, j as usize);
                if map[i<<b|j] == b'X' || v[i<<b|j] { continue; }
                v[i<<b|j] = true; q[it] = i<<b|j; it = it+1&QMASK;
            }
        }
        if flag { break; }
    }
    if flag { ret } else { 0 }
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let b = m.next_power_of_two().trailing_zeros();
    let mut map = [b'X'; MAX];
    for i in 0..n {
        let s = next::<String>();
        for (j, c) in s.into_bytes().into_iter().enumerate() {
            map[i<<b|j] = c;
        }
    }

    let mut ans = 0;
    let c = (1<<b)-1;
    let mut q = [0; QSZ];
    for i in 0..n { for j in 0..m {
        if map[i<<b|j] == b'#' {
            ans = ans.max(bfs(&map, &mut q, i, j, b, c, n, m));
        }
    }}
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