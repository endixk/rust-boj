// BOJ 6593 [Dungeon Master]
const DX: [i32; 6] = [0, 0, 0, 0, 1, -1];
const DY: [i32; 6] = [0, 0, 1, -1, 0, 0];
const DZ: [i32; 6] = [1, -1, 0, 0, 0, 0];
pub fn main() { read();
    'x: loop {
        let (l, r, _) = (next::<usize>(), next::<usize>(), next::<usize>());
        if l == 0 { break; }

        let (mut s, mut e) = (0, 0);
        let mut map = vec![false; 1<<15];
        for i in 1..=l {
            for j in 1..=r {
                for (k, c) in next::<String>().chars().enumerate() {
                    let x = i<<10 | j<<5 | k+1;
                    if c == 'S' { s = x; map[x] = true; }
                    else if c == 'E' { e = x; map[x] = true; }
                    else if c == '.' { map[x] = true; }
                }
            }
        }

        let mut v = vec![false; 1<<15];
        let mut q = vec![s];
        let mut d = 0;
        v[s] = true;
        while !q.is_empty() {
            let mut nq = vec![];
            while let Some(x) = q.pop() {
                if x == e { println!("Escaped in {} minute(s).", d); continue 'x; }
                let (i, j, k) = (x>>10, x>>5&0x1f, x&0x1f);
                for ((&dx, &dy), &dz) in DX.iter().zip(DY.iter()).zip(DZ.iter()) {
                    let (ni, nj, nk) = (i as i32 + dx, j as i32 + dy, k as i32 + dz);
                    let nx = (ni as usize)<<10 | (nj as usize)<<5 | nk as usize;
                    if map[nx] && !v[nx] {
                        v[nx] = true;
                        nq.push(nx);
                    }
                }
            }
            q = nq; d += 1;
        }
        println!("Trapped!");
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