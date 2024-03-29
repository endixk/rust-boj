// BOJ 23747 [Wards]
// Supported by GitHub Copilot

const DX: [i32; 4] = [0, 0, 1, -1];
const DY: [i32; 4] = [1, -1, 0, 0];
fn fill(a: &Vec<Vec<u8>>, b: &mut Vec<Vec<bool>>, c: u8, i: usize, j: usize, n: usize, m: usize) {
    let mut q = std::collections::VecDeque::new();
    q.push_back((i, j)); b[i][j] = true;
    while let Some((x, y)) = q.pop_front() {
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let nx = x as i32 + dx; let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 { continue; }
            let (nx, ny) = (nx as usize, ny as usize);
            if b[nx][ny] || a[nx][ny] != c { continue; }
            b[nx][ny] = true; q.push_back((nx, ny));
        }
    }
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let mut a = vec![vec![0; m]; n];
    for i in 0..n {
        let s = next::<String>();
        for (j, c) in s.chars().enumerate() {
            a[i][j] = c as u8 - 'a' as u8;
        }
    }

    let mut b = vec![vec![false; m]; n];
    let (mut x, mut y) = (next::<usize>()-1, next::<usize>()-1);
    let cmd = next::<String>();
    for c in cmd.chars() {
        match c {
            'W' if !b[x][y] => fill(&a, &mut b, a[x][y], x, y, n, m),
            'U' => x -= 1, 'D' => x += 1, 'L' => y -= 1, 'R' => y += 1,
            _ => (),
        }
    }

    for i in 0..n {
        for j in 0..m {
            let d = if i > x { i - x } else { x - i } + if j > y { j - y } else { y - j };
            print!("{}", if !b[i][j] && d > 1 { '#' } else { '.' });
        } println!();
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