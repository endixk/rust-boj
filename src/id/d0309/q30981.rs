use std::collections::VecDeque;
const DX: [i32; 9] = [0, 1, 1, 1, 0, -1, -1, -1, 0];
const DY: [i32; 9] = [1, 1, 0, -1, -1, -1, 0, 1, 0];
const MAX: usize = 1<<16;
fn valid(map: &[char; MAX], n: usize, m: usize, k: usize, t: usize) -> bool {
    let mut fw = 0;
    let mut v = vec![false; MAX];
    let mut q = VecDeque::new();
    q.push_back((0, 0)); v[0] = true;
    'f: while !q.is_empty() {
        let sz = q.len();
        for _ in 0..sz {
            let (x, y) = q.pop_front().unwrap();
            if x == n-1 && y == m-1 { break 'f; }
            for (&dx, &dy) in DX.iter().take(8).zip(DY.iter().take(8)) {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
                let (nx, ny) = (nx as usize, ny as usize);
                if v[nx<<8|ny] || map[nx<<8|ny] == '#' { continue; }
                q.push_back((nx, ny)); v[nx<<8|ny] = true;
            }
        }
        fw += 1;
    }
    if !v[n-1<<8|m-1] { return false; }
    let bw = fw;
    fw = fw * 10 - 5 + k;
    fw += (15 - fw % 10) % 10;
    fw += (bw - 1) * 10;
    fw <= t
}
fn generate(map: &[char; MAX], n: usize, m: usize, t: usize, r: &Vec<Vec<(usize, usize)>>, reach: &mut [bool; MAX]) {
    for i in 0..n { for j in 0..m {
        reach[i<<8|j] = map[i<<8|j] != '#';
    }}
    for path in r {
        let k = (t / 10) % path.len();
        let (x, y) = path[k];
        for (&dx, &dy) in DX.iter().zip(DY.iter()) {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
            let (nx, ny) = (nx as usize, ny as usize);
            reach[nx<<8|ny] = false;
        }
    }
    reach[0] = true; reach[n-1<<8|m-1] = true;
}
pub fn main() { read();
    let (n, m) = (next::<usize>(), next::<usize>());
    let (l, z, k) = (next::<usize>(), next::<usize>(), next::<usize>());
    if n.max(m) > 200 { println!("SAD"); return; }

    let mut map = [' '; MAX];
    for i in 0..n {
        let s = next::<String>();
        for (j, c) in s.chars().enumerate() {
            map[i<<8|j] = c;
        }
    }
    if !valid(&map, n, m, k, z) { println!("SAD"); return; }

    let mut r = vec![vec![]; l];
    for i in 0..l {
        for _ in 0..next() {
            let (x, y) = (next::<usize>(), next::<usize>());
            r[i].push((x-1, y-1));
        }
    }

    let mut reach = [false; MAX];
    let mut locs = vec![(0, 0)];
    let mut t = 5;
    'f: while !locs.is_empty() {
        let mut nlocs = vec![];
        let mut v = [false; MAX];
        generate(&map, n, m, t, &r, &mut reach);
        for &(x, y) in &locs {
            for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
                let (nx, ny) = (nx as usize, ny as usize);
                if v[nx<<8|ny] || !reach[nx<<8|ny] { continue; }
                nlocs.push((nx, ny)); v[nx<<8|ny] = true;
            }
        }
        locs = nlocs;
        if v[n-1<<8|m-1] { break 'f; }
        t += 10; if t > z { break 'f; }
    }
    t += k;
    t += (15 - t % 10) % 10;
    if t > z { println!("SAD"); return; }

    locs = vec![(n-1, m-1)];
    'r: while !locs.is_empty() {
        let mut nlocs = vec![];
        let mut v = [false; MAX];
        generate(&map, n, m, t, &r, &mut reach);
        for &(x, y) in &locs {
            for (&dx, &dy) in DX.iter().zip(DY.iter()) {
                let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                if nx < 0 || nx >= n as i32 || ny < 0 || ny >= m as i32 { continue; }
                let (nx, ny) = (nx as usize, ny as usize);
                if v[nx<<8|ny] || !reach[nx<<8|ny] { continue; }
                nlocs.push((nx, ny)); v[nx<<8|ny] = true;
            }
        }
        locs = nlocs;
        if v[0] { break 'r; }
        t += 10; if t > z { break 'r; }
    }
    if t > z { println!("SAD"); return; }
    println!("YUMMY");
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