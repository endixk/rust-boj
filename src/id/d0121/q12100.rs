// BOJ 12100 [2048 (Easy)]
// Supported by GitHub Copilot

use std::io;
use std::io::prelude::*;

fn read(si: &mut io::BufReader<io::StdinLock>) -> String {
    let mut s = String::new();
    si.read_to_string(&mut s).unwrap();
    s
}

fn next<T>(it: &mut std::str::SplitAsciiWhitespace) -> T where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug {
    it.next().unwrap().parse().unwrap()
}

struct Block {
    value: i32,
    row: usize,
    col: usize,
}

impl Block {
    fn new(value: i32, row: usize, col: usize) -> Self {
        Self { value, row, col }
    }
}
impl Clone for Block {
    fn clone(&self) -> Self {
        Self::new(self.value, self.row, self.col)
    }
}

fn move_l(mut blocks: Vec<Block>, size: usize) -> Vec<Block> {
    blocks.sort_by(|a, b| a.col.cmp(&b.col));
    let mut border = Vec::<(i32, usize)>::new(); // (value, col)
    for _ in 0..size {
        border.push((-1, 0));
    }

    let mut next = Vec::<Block>::new();
    for block in blocks {
        let row = block.row;
        if border[row].0 < 0 {
            border[row].0 = block.value;
        } else if border[row].0 == block.value {
            next.push(Block::new(block.value * 2, row, border[row].1));
            border[row].0 = -1;
            border[row].1 += 1;
        } else {
            next.push(Block::new(border[row].0, row, border[row].1));
            border[row].0 = block.value;
            border[row].1 += 1;
        }
    }

    for row in 0..size {
        if border[row].0 > 0 {
            next.push(Block::new(border[row].0, row, border[row].1));
        }
    }

    next
}
fn move_r(mut blocks: Vec<Block>, size: usize) -> Vec<Block> {
    blocks.sort_by(|a, b| b.col.cmp(&a.col));
    let mut border = Vec::<(i32, usize)>::new(); // (value, col)
    for _ in 0..size {
        border.push((-1, size - 1));
    }

    let mut next = Vec::<Block>::new();
    for block in blocks {
        let row = block.row;
        if border[row].0 < 0 {
            border[row].0 = block.value;
        } else if border[row].0 == block.value {
            next.push(Block::new(block.value * 2, row, border[row].1));
            border[row].0 = -1;
            border[row].1 -= 1;
        } else {
            next.push(Block::new(border[row].0, row, border[row].1));
            border[row].0 = block.value;
            border[row].1 -= 1;
        }
    }

    for row in 0..size {
        if border[row].0 > 0 {
            next.push(Block::new(border[row].0, row, border[row].1));
        }
    }

    next
}
fn move_u(mut blocks: Vec<Block>, size: usize) -> Vec<Block> {
    blocks.sort_by(|a, b| a.row.cmp(&b.row));
    let mut border = Vec::<(i32, usize)>::new(); // (value, row)
    for _ in 0..size {
        border.push((-1, 0));
    }

    let mut next = Vec::<Block>::new();
    for block in blocks {
        let col = block.col;
        if border[col].0 < 0 {
            border[col].0 = block.value;
        } else if border[col].0 == block.value {
            next.push(Block::new(block.value * 2, border[col].1, col));
            border[col].0 = -1;
            border[col].1 += 1;
        } else {
            next.push(Block::new(border[col].0, border[col].1, col));
            border[col].0 = block.value;
            border[col].1 += 1;
        }
    }

    for col in 0..size {
        if border[col].0 > 0 {
            next.push(Block::new(border[col].0, border[col].1, col));
        }
    }

    next
}
fn move_d(mut blocks: Vec<Block>, size: usize) -> Vec<Block> {
    blocks.sort_by(|a, b| b.row.cmp(&a.row));
    let mut border = Vec::<(i32, usize)>::new(); // (value, row)
    for _ in 0..size {
        border.push((-1, size - 1));
    }

    let mut next = Vec::<Block>::new();
    for block in blocks {
        let col = block.col;
        if border[col].0 < 0 {
            border[col].0 = block.value;
        } else if border[col].0 == block.value {
            next.push(Block::new(block.value * 2, border[col].1, col));
            border[col].0 = -1;
            border[col].1 -= 1;
        } else {
            next.push(Block::new(border[col].0, border[col].1, col));
            border[col].0 = block.value;
            border[col].1 -= 1;
        }
    }

    for col in 0..size {
        if border[col].0 > 0 {
            next.push(Block::new(border[col].0, border[col].1, col));
        }
    }

    next
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());

    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();
    let n: usize = next(&mut it);

    let mut blocks = Vec::<Block>::new();
    for row in 0..n {
        for col in 0..n {
            let value = next(&mut it);
            if value > 0 {
                blocks.push(Block::new(value, row, col));
            }
        }
    }

    let mut ans = 0;
    for i in 0..(1 << 10) {
        let mut next = blocks.clone();
        for j in 0..5 {
            next = match (i >> (j * 2)) & 3 {
                0 => move_l(next, n),
                1 => move_r(next, n),
                2 => move_u(next, n),
                3 => move_d(next, n),
                _ => unreachable!(),
            };

            let vmax = next.iter().map(|b| b.value).max().unwrap();
            if vmax << (5 - j) <= ans {
                break;
            }
        }
        for block in next {
            ans = ans.max(block.value);
        }
    }

    writeln!(so, "{}", ans).unwrap();
}
