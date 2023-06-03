// BOJ 17143 [The King of Fishing]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read<T>(si: &mut T) -> String where T: Read {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

#[derive(Clone, Copy)]
struct Shark {
    z: u32, // size
    d: u8, // direction; 1: up, 2: down, 3: right, 4: left
    s: i16, // speed
    i: i16, // row
    j: i16, // column
}
impl Shark {
    fn next(&self, r: i16, c: i16) -> (u8, i16, i16) {
        let (mut d, mut i, mut j) = (self.d, self.i, self.j);
        return if d == 1 {
            i = (r - i - 1) + self.s;
            if i >= r {
                d = (2 - (i - r) / (r - 1) % 2) as u8;
                i = if d == 1 {
                    (i - r) % (r - 1) + 1
                } else {
                    r - (i - r) % (r - 1) - 2
                };
            }
            (d, r - i - 1, j)
        } else if d == 2 {
            i += self.s;
            if i >= r {
                d = ((i - r) / (r - 1) % 2 + 1) as u8;
                i = if d == 2 {
                    (i - r) % (r - 1) + 1
                } else {
                    r - (i - r) % (r - 1) - 2
                };
            }
            (d, i, j)
        } else if d == 3 {
            j += self.s;
            if j >= c {
                d = (4 - (j - c) / (c - 1) % 2) as u8;
                j = if d == 3 {
                    (j - c) % (c - 1) + 1
                } else {
                    c - (j - c) % (c - 1) - 2
                };
            }
            (d, i, j)
        } else {
            j = (c - j - 1) + self.s;
            if j >= c {
                d = ((j - c) / (c - 1) % 2 + 3) as u8;
                j = if d == 4 {
                    (j - c) % (c - 1) + 1
                } else {
                    c - (j - c) % (c - 1) - 2
                };
            }
            (d, i, c - j - 1)
        }
    }
}

fn catch(board: &mut [Option<Shark>; 1<<14], j: usize, r: usize) -> u32 {
    for i in 0..r {
        if let Some(shark) = board[i<<7|j].take() {
            return shark.z;
        }
    }
    0
}

fn go(board: [Option<Shark>; 1<<14], r: usize, c: usize) -> [Option<Shark>; 1<<14] {
    let mut ret = [None::<Shark>; 1<<14];
    for i in 0..r { for j in 0..c {
        if let Some(shark) = board[i<<7|j] {
            let (d, i, j) = shark.next(r as i16, c as i16);
            let (x, y) = (i as usize, j as usize);
            if let Some(old_shark) = ret[x<<7|y] {
                if old_shark.z < shark.z {
                    ret[x<<7|y] = Some(Shark { z: shark.z, d, s: shark.s, i, j });
                }
            } else {
                ret[x<<7|y] = Some(Shark { z: shark.z, d, s: shark.s, i, j });
            }
        }
    }}
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (r, c, m) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut board = [None; 1<<14];
    for _ in 0..m {
        let (i, j, s, d, z) = (
            next::<usize>(&mut it) - 1,
            next::<usize>(&mut it) - 1,
            next::<i16>(&mut it),
            next::<u8>(&mut it),
            next::<u32>(&mut it));
        board[i<<7|j] = Some(Shark { z, d, s, i: i as i16, j: j as i16 });
    }

    let mut ans = 0;
    for j in 0..c {
        ans += catch(&mut board, j, r);
        board = go(board, r, c);
    }
    println!("{}", ans);
}
