// BOJ 20549 [Duck Problem]
const INF: u32 = 0x3f3f3f3f;
const KI: [i32; 8] = [2, 1, -1, -2, -2, -1, 1, 2];
const KJ: [i32; 8] = [1, 2, 2, 1, -1, -2, -2, -1];
pub fn main() { read();
    let n = next::<usize>();
    let (a, b, c) = (next::<u32>(), next::<u32>(), next::<u32>());
    let (x, y, m) = (next::<usize>(), next::<usize>(), next::<usize>());

    let mut food = vec![m; 1<<12];
    for i in 0..m {
        let (p, q) = (next::<usize>(), next::<usize>());
        food[p<<6|q] = i;
    }

    let mut dist = vec![INF; 1<<16];
    dist[x<<10|y<<4] = 0;
    let mut q = vec![x<<10|y<<4];
    while !q.is_empty() {
        let mut nq = vec![];
        for &x in &q {
            let (i, j, k) = (x>>10, x>>4&0x3f, x&0xf);
            // Knight's moves
            for (&di, &dj) in KI.iter().zip(KJ.iter()) {
                let (x, y) = (i as i32 + di, j as i32 + dj);
                if x < 0 || y < 0 || x >= n as i32 || y >= n as i32 { continue; }
                let (x, y) = (x as usize, y as usize);
                let z = if food[x<<6|y] != m { k | 1 << food[x<<6|y] } else { k };
                if dist[x<<10|y<<4|z] > dist[i<<10|j<<4|k] + a {
                    dist[x<<10|y<<4|z] = dist[i<<10|j<<4|k] + a;
                    nq.push(x<<10|y<<4|z);
                }
            }

            // Bishop's moves
            for d in 1.. {
                let (x, y) = (i as i32 + d, j as i32 + d);
                if x >= n as i32 || y >= n as i32 { break; }
                let (x, y) = (x as usize, y as usize);
                let z = if food[x<<6|y] != m { k | 1 << food[x<<6|y] } else { k };
                if dist[x<<10|y<<4|z] > dist[i<<10|j<<4|k] + b {
                    dist[x<<10|y<<4|z] = dist[i<<10|j<<4|k] + b;
                    nq.push(x<<10|y<<4|z);
                }
            }
            for d in 1.. {
                let (x, y) = (i as i32 + d, j as i32 - d);
                if x >= n as i32 || y < 0 { break; }
                let (x, y) = (x as usize, y as usize);
                let z = if food[x<<6|y] != m { k | 1 << food[x<<6|y] } else { k };
                if dist[x<<10|y<<4|z] > dist[i<<10|j<<4|k] + b {
                    dist[x<<10|y<<4|z] = dist[i<<10|j<<4|k] + b;
                    nq.push(x<<10|y<<4|z);
                }
            }
            for d in 1.. {
                let (x, y) = (i as i32 - d, j as i32 + d);
                if x < 0 || y >= n as i32 { break; }
                let (x, y) = (x as usize, y as usize);
                let z = if food[x<<6|y] != m { k | 1 << food[x<<6|y] } else { k };
                if dist[x<<10|y<<4|z] > dist[i<<10|j<<4|k] + b {
                    dist[x<<10|y<<4|z] = dist[i<<10|j<<4|k] + b;
                    nq.push(x<<10|y<<4|z);
                }
            }
            for d in 1.. {
                let (x, y) = (i as i32 - d, j as i32 - d);
                if x < 0 || y < 0 { break; }
                let (x, y) = (x as usize, y as usize);
                let z = if food[x<<6|y] != m { k | 1 << food[x<<6|y] } else { k };
                if dist[x<<10|y<<4|z] > dist[i<<10|j<<4|k] + b {
                    dist[x<<10|y<<4|z] = dist[i<<10|j<<4|k] + b;
                    nq.push(x<<10|y<<4|z);
                }
            }

            // Rook's moves
            for x in 0..i {
                let z = if food[x<<6|j] != m { k | 1 << food[x<<6|j] } else { k };
                if dist[x<<10|j<<4|z] > dist[i<<10|j<<4|k] + c {
                    dist[x<<10|j<<4|z] = dist[i<<10|j<<4|k] + c;
                    nq.push(x<<10|j<<4|z);
                }
            }
            for x in i+1..n {
                let z = if food[x<<6|j] != m { k | 1 << food[x<<6|j] } else { k };
                if dist[x<<10|j<<4|z] > dist[i<<10|j<<4|k] + c {
                    dist[x<<10|j<<4|z] = dist[i<<10|j<<4|k] + c;
                    nq.push(x<<10|j<<4|z);
                }
            }
            for y in 0..j {
                let z = if food[i<<6|y] != m { k | 1 << food[i<<6|y] } else { k };
                if dist[i<<10|y<<4|z] > dist[i<<10|j<<4|k] + c {
                    dist[i<<10|y<<4|z] = dist[i<<10|j<<4|k] + c;
                    nq.push(i<<10|y<<4|z);
                }
            }
            for y in j+1..n {
                let z = if food[i<<6|y] != m { k | 1 << food[i<<6|y] } else { k };
                if dist[i<<10|y<<4|z] > dist[i<<10|j<<4|k] + c {
                    dist[i<<10|y<<4|z] = dist[i<<10|j<<4|k] + c;
                    nq.push(i<<10|y<<4|z);
                }
            }
        }
        q = nq;
    }

    let mut ans = INF;
    for i in 0..n {
        for j in 0..n {
            ans = ans.min(dist[i<<10|j<<4|(1<<m)-1]);
        }
    }
    println!("{}", ans);
}

macro_rules! println { ($($t:tt)*) => {SO.with(|c| writeln!(c.borrow_mut(), $($t)*).unwrap())};}
macro_rules! print   { ($($t:tt)*) => {SO.with(|c| write!  (c.borrow_mut(), $($t)*).unwrap())};}
use println; use print;
use std::{io::*, cell::*, str::*, fmt::Debug, ptr::addr_of_mut};
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