// BOJ 30702 [Flag Coloring]
// Supported by GitHub Copilot

const DX: [i32; 4] = [0, 0, 1, -1];
const DY: [i32; 4] = [1, -1, 0, 0];
fn fill(f: &mut Vec<Vec<u8>>, v: &mut Vec<Vec<bool>>, n: usize, m: usize, i: usize, j: usize, k: u8, c: u8) {
    let mut q = std::collections::VecDeque::new();
    q.push_back((i, j)); v[i][j] = true;
    while let Some((x, y)) = q.pop_front() {
        f[x][y] = c;
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
            let (nx, ny) = (nx as usize, ny as usize);
            if v[nx][ny] || f[nx][ny] != k { continue; }
            q.push_back((nx, ny)); v[nx][ny] = true;
        }
    }
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let (mut f, mut g) = (vec![vec![0; m]; n], vec![vec![0; m]; n]);
    for i in 0..n {
        let s = next::<String>();
        for (j, c) in s.chars().enumerate() {
            f[i][j] = c as u8 - b'A';
        }
    }
    for i in 0..n {
        let s = next::<String>();
        for (j, c) in s.chars().enumerate() {
            g[i][j] = c as u8 - b'A';
        }
    }

    let mut v = vec![vec![false; m]; n];
    for i in 0..n { for j in 0..m {
        if v[i][j] && f[i][j] != g[i][j] {
            println!("NO"); return;
        }
        if v[i][j] { continue; }
        let k = f[i][j];
        fill(&mut f, &mut v, n, m, i, j, k, g[i][j]);
    }}
    println!("YES");
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