// BOJ 1525 [Puzzle]
// Supported by GitHub Copilot

use std::io::{self, BufRead};
use std::collections::{VecDeque, HashSet};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Board {
    blocks: [u8; 9],
    zloc: usize,
}
fn swap(blocks: &[u8; 9], i: usize, j: usize) -> [u8; 9] {
    let mut b = *blocks;
    b[i] = blocks[j];
    b[j] = blocks[i];
    b
}
impl Board {
    fn next(&self) -> Vec<Board> {
        let mut v = Vec::new();
        match self.zloc {
            0 => {
                v.push(Board { blocks: swap(&self.blocks, 0, 1), zloc: 1 });
                v.push(Board { blocks: swap(&self.blocks, 0, 3), zloc: 3 });
            },
            1 => {
                v.push(Board { blocks: swap(&self.blocks, 1, 0), zloc: 0 });
                v.push(Board { blocks: swap(&self.blocks, 1, 2), zloc: 2 });
                v.push(Board { blocks: swap(&self.blocks, 1, 4), zloc: 4 });
            },
            2 => {
                v.push(Board { blocks: swap(&self.blocks, 2, 1), zloc: 1 });
                v.push(Board { blocks: swap(&self.blocks, 2, 5), zloc: 5 });
            },
            3 => {
                v.push(Board { blocks: swap(&self.blocks, 3, 0), zloc: 0 });
                v.push(Board { blocks: swap(&self.blocks, 3, 4), zloc: 4 });
                v.push(Board { blocks: swap(&self.blocks, 3, 6), zloc: 6 });
            },
            4 => {
                v.push(Board { blocks: swap(&self.blocks, 4, 1), zloc: 1 });
                v.push(Board { blocks: swap(&self.blocks, 4, 3), zloc: 3 });
                v.push(Board { blocks: swap(&self.blocks, 4, 5), zloc: 5 });
                v.push(Board { blocks: swap(&self.blocks, 4, 7), zloc: 7 });
            },
            5 => {
                v.push(Board { blocks: swap(&self.blocks, 5, 2), zloc: 2 });
                v.push(Board { blocks: swap(&self.blocks, 5, 4), zloc: 4 });
                v.push(Board { blocks: swap(&self.blocks, 5, 8), zloc: 8 });
            },
            6 => {
                v.push(Board { blocks: swap(&self.blocks, 6, 3), zloc: 3 });
                v.push(Board { blocks: swap(&self.blocks, 6, 7), zloc: 7 });
            },
            7 => {
                v.push(Board { blocks: swap(&self.blocks, 7, 4), zloc: 4 });
                v.push(Board { blocks: swap(&self.blocks, 7, 6), zloc: 6 });
                v.push(Board { blocks: swap(&self.blocks, 7, 8), zloc: 8 });
            },
            8 => {
                v.push(Board { blocks: swap(&self.blocks, 8, 5), zloc: 5 });
                v.push(Board { blocks: swap(&self.blocks, 8, 7), zloc: 7 });
            },
            _ => unreachable!(),
        }
        v
    }
}

pub fn main() {
    let v = io::stdin().lock().lines().take(3).map(|line| {
        line.unwrap().split_whitespace().map(|n| n.parse::<u8>().unwrap()).collect::<Vec<_>>()
    }).flatten().collect::<Vec<_>>();
    let zloc = v.iter().position(|&n| n == 0).unwrap();
    let board = Board { blocks: [v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8]], zloc };

    let mut q = VecDeque::new();
    let mut vis = HashSet::new();
    q.push_back((board, 0));
    vis.insert(board);
    while let Some((b, d)) = q.pop_front() {
        if b.blocks == [1, 2, 3, 4, 5, 6, 7, 8, 0] {
            println!("{}", d);
            return;
        }
        for nb in b.next() {
            if !vis.contains(&nb) {
                q.push_back((nb, d + 1));
                vis.insert(nb);
            }
        }
    }
    println!("-1");
}
