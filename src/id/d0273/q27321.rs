// BOJ 27321 [Analyzing Tetris]
const MOD: usize = 998_244_353;
const BLOCKS: [[(usize,usize); 4]; 19] = [
    [(0,0),(0,1),(0,2),(0,3)], // I1
    [(0,0),(1,0),(2,0),(3,0)], // I2
    [(0,1),(1,1),(2,1),(2,0)], // J1
    [(0,0),(1,0),(1,1),(1,2)], // J2
    [(0,0),(0,1),(1,0),(2,0)], // J3
    [(0,0),(0,1),(0,2),(1,2)], // J4
    [(0,0),(1,0),(2,0),(2,1)], // L1
    [(0,0),(0,1),(0,2),(1,0)], // L2
    [(0,0),(0,1),(1,1),(2,1)], // L3
    [(0,2),(1,0),(1,1),(1,2)], // L4
    [(0,0),(0,1),(1,0),(1,1)], // O
    [(0,1),(0,2),(1,0),(1,1)], // S1
    [(0,0),(1,0),(1,1),(2,1)], // S2
    [(0,0),(0,1),(0,2),(1,1)], // T1
    [(0,1),(1,0),(1,1),(2,1)], // T2
    [(0,1),(1,0),(1,1),(1,2)], // T3
    [(0,0),(1,0),(1,1),(2,0)], // T4
    [(0,0),(0,1),(1,1),(1,2)], // Z1
    [(0,1),(1,0),(1,1),(2,0)], // Z2
];

fn valid(p: &Vec<u16>, h: usize, w: usize, i: usize, j: usize, k: usize) -> bool {
    for &(di, dj) in &BLOCKS[k] {
        if i+di >= h || j+dj >= w { return false; }
        if p[i+di] & (1 << (j+dj)) != 0 { return false; }
    }
    true
}
fn plug(p: &Vec<u16>, h: usize, w: usize, mask: u16, r: usize, i: usize, j: usize, k: usize) -> Option<usize> {
    if !valid(p, h, w, i, j, k) { return None; }
    if valid(p, h, w, i+1, j, k) { return None; }
    let mut z = vec![false; h];
    for i in 0..h { if p[i] == mask { z[i] = true; }}
    let mut q = p.clone();
    for &(di, dj) in &BLOCKS[k] {
        q[i+di] |= 1 << (j+dj);
    }

    let mut ret = 0;
    let mut c = 0;
    for i in 0..h {
        if !z[i] && q[i] == mask { c += 1; }
        else {
            ret = (ret << w) % MOD;
            ret = (ret + q[i] as usize) % MOD;
        }
    }
    if c != r { return None; }
    Some(ret)
}
pub fn main() { read();
    let (h, w) = (next::<usize>(), next::<usize>());
    let mask = (1 << w) - 1;
    let mut p = vec![mask; h+8];
    let mut pc = 0;
    for i in 4..h+4 {
        p[i] = 0;
        for (j, c) in next::<String>().into_bytes().into_iter().enumerate() {
            if c == b'#' {
                p[i] |= 1 << j;
                pc += 1;
            }
        }
    }

    let mut q = vec![mask; h+8];
    let mut qc = 0;
    for i in 4..h+4 {
        q[i] = 0;
        for (j, c) in next::<String>().into_bytes().into_iter().enumerate() {
            if c == b'#' {
                q[i] |= 1 << j;
                qc += 1;
            }
        }
    }

    if pc + 4 < qc { println!("X"); return; }
    if (pc + 4 - qc) as usize % w != 0 { println!("X"); return; }
    let r = (pc + 4 - qc) as usize / w;
    if r > 4 { println!("X"); return; }

    let mut d = 4;
    loop {
        if p[d] != q[d+r] { break; }
        d += 1;
    }

    let mut qhash = 0;
    for i in 0..4 {
        qhash = (qhash << w) % MOD;
        qhash = (qhash + q[i] as usize) % MOD;
    }
    for i in 4+r..h+8 {
        qhash = (qhash << w) % MOD;
        qhash = (qhash + q[i] as usize) % MOD;
    }

    let mut phash = 0;
    for i in 0..d-4 {
        phash = (phash << w) % MOD;
        phash = (phash + p[i] as usize) % MOD;
    }
    for _ in 0..9-r { phash = (phash << w) % MOD; }
    let mut factor = 1;
    for i in d+5..h+8 {
        factor = (factor << w) % MOD;
        phash = (phash << w) % MOD;
        phash = (phash + p[i] as usize) % MOD;
    }

    let sub = &p[d-4..d+5].to_vec();
    let mut ans = vec![];
    'x: for v in vec![vec![0, 1], vec![2, 3, 4, 5], vec![6, 7, 8, 9], vec![10], vec![11, 12], vec![13, 14, 15, 16], vec![17, 18]] {
        for k in v {
            for i in 0..9 { for j in 0..w {
                if let Some(x) = plug(&sub, 9, w, mask, r, i, j, k) {
                    if (phash + x * factor) % MOD == qhash {
                        match k {
                            k if k < 2 => ans.push('I'),
                            k if k < 6 => ans.push('J'),
                            k if k < 10 => ans.push('L'),
                            k if k < 11 => ans.push('O'),
                            k if k < 13 => ans.push('S'),
                            k if k < 17 => ans.push('T'),
                            _ => ans.push('Z'),
                        }
                        continue 'x;
                    }
                }
            }}
        }
    }

    if ans.is_empty() { println!("X"); return; }
    println!("{}", ans.into_iter().collect::<String>());
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
    SI.with(|c| c.borrow_mut().read_to_string(&mut BUF).unwrap());
    IT = Some(BUF.split_ascii_whitespace());
}}
fn next<T: FromStr>() -> T where <T as FromStr>::Err: Debug {
    unsafe { IT.as_mut().unwrap().next().unwrap().parse().unwrap() }
}