// BOJ 30894 [Funhouse Escape]
// Supported by GitHub Copilot

pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let (sx, sy) = (next::<usize>(), next::<usize>());
    let (ex, ey) = (next::<usize>(), next::<usize>());

    let mut b = vec![vec![vec![4; 4]; m]; n];
    for i in 0..n {
        let s = next::<String>();
        for (j, c) in s.chars().enumerate() {
            b[i][j][0] = match c {
                '.' => 5, '0' => 0, '1' => 1, '2' => 2, '3' => 3, _ => 4
            };
            if b[i][j][0] < 4 {
                b[i][j][1] = (b[i][j][0] + 1) % 4;
                b[i][j][2] = (b[i][j][0] + 2) % 4;
                b[i][j][3] = (b[i][j][0] + 3) % 4;
            } else {
                b[i][j][1] = b[i][j][0];
                b[i][j][2] = b[i][j][0];
                b[i][j][3] = b[i][j][0];
            }
        }
    }

    for i in 0..n { for j in 0..m { for k in 0..4 {
        if b[i][j][k] == 0 {
            for t in j+1..m {
                if b[i][t][k] < 5 { break; }
                b[i][t][k] = 6;
            }
        } else if b[i][j][k] == 1 {
            for t in i+1..n {
                if b[t][j][k] < 5 { break; }
                b[t][j][k] = 6;
            }
        } else if b[i][j][k] == 2 {
            for t in (0..j).rev() {
                if b[i][t][k] < 5 { break; }
                b[i][t][k] = 6;
            }
        } else if b[i][j][k] == 3 {
            for t in (0..i).rev() {
                if b[t][j][k] < 5 { break; }
                b[t][j][k] = 6;
            }
        }
    }}}

    const DX: [i32; 5] = [0, 1, 0, -1, 0];
    const DY: [i32; 5] = [0, 0, 1, 0, -1];
    let mut d = 0;
    let mut q = std::collections::VecDeque::new();
    let mut v = vec![vec![vec![false; 4]; m]; n];
    q.push_back((sx-1, sy-1, 0)); v[sx-1][sy-1][0] = true;
    while !q.is_empty() {
        let sz = q.len();
        for _ in 0.. sz {
            let (x, y, z) = q.pop_front().unwrap();
            if x == ex - 1 && y == ey - 1 { println!("{}", d); return; }
            for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
                let (nx, ny) = (nx as usize, ny as usize);
                if b[nx][ny][(z + 1) % 4] == 5 && !v[nx][ny][(z + 1) % 4] {
                    q.push_back((nx, ny, (z + 1) % 4));
                    v[nx][ny][(z + 1) % 4] = true;
                }
            }
        }
        d += 1;
    }
    println!("GG");
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