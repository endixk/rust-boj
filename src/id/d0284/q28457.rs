use std::io::{self, Read};
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

fn special(i: usize, n: usize) -> u8 {
    return if i == 0 { 1 } // start
    else if i == n-1 { 2 } // island
    else if i == 2*n-2 { 3 } // fund
    else if i == 3*n-3 { 4 } // space travel
    else { 0 }
}
struct Board {
    n: usize,
    x: usize,
    city: Vec<bool>,
    price: Vec<i32>,
    fund: i32,
    keys: Vec<(u8, i32)>,
    key: usize,
}
impl Board {
    fn new(n: usize, info: Vec<(char, i32)>, keys: Vec<(u8, i32)>) -> Self {
        let x = 4*n - 4;
        let mut city = vec![false; x];
        let mut price = vec![0; x];

        let (mut i, mut j) = (0, 0);
        while i < x {
            if special(i, n) == 0 {
                let (c, p) = info[j];
                if c == 'L' {
                    city[i] = true;
                    price[i] = p;
                }
                j += 1;
            }
            i += 1;
        }
        Board { n, x, city, price, fund: 0, keys, key: 0 }
    }
    fn get_key(&mut self) -> (u8, i32) {
        let (k, p) = self.keys[self.key];
        self.key = (self.key + 1) % self.keys.len();
        (k, p)
    }
}

struct Player {
    pos: usize,
    money: i32,
    salary: i32,
    locked: u8,
}
impl Player {
    fn new(s: i32, w: i32) -> Self {
        Player { pos: 0, money: s, salary: w, locked: 0 }
    }
    fn go(&mut self, board: &mut Board, i: usize, j: usize) {
        if self.locked > 0 {
            if i == j {
                self.locked = 0;
                return;
            } else {
                self.locked -= 1;
                return;
            }
        }

        self.pos += i+j;
        while self.pos >= board.x {
            self.pos -= board.x;
            self.money += self.salary;
        }

        if special(self.pos, board.n) == 1 {
            return;
        }
        if special(self.pos, board.n) == 2 {
            self.locked = 3;
            return;
        }
        if special(self.pos, board.n) == 3 {
            self.money += board.fund;
            board.fund = 0;
            return;
        }
        if special(self.pos, board.n) == 4 {
            self.pos = 0;
            self.money += self.salary;
            return;
        }

        if board.city[self.pos] {
            if self.money >= board.price[self.pos] {
                self.money -= board.price[self.pos];
                board.price[self.pos] = 0;
            }
        } else {
            let (k, p) = board.get_key();
            if k == 1 {
                self.money += p;
            } else if k == 2 {
                self.money -= p;
            } else if k == 3 {
                self.money -= p;
                board.fund += p;
            } else {
                self.go(board, 0, p as usize);
            }
        }
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, s, w, g) = (
        next::<usize>(&mut it),
        next::<i32>(&mut it),
        next::<i32>(&mut it),
        next::<usize>(&mut it));
    let keys = (0..g).map(|_| {
        (next::<u8>(&mut it), next::<i32>(&mut it))
    }).collect::<Vec<_>>();
    let info = (0..4*n-8).map(|_| {
        let c = next::<char>(&mut it);
        let p = if c == 'L' { next::<i32>(&mut it) } else { 0 };
        (c, p)
    }).collect::<Vec<_>>();

    let mut board = Board::new(n, info, keys);
    let mut player = Player::new(s, w);
    for _ in 0..next(&mut it) {
        let (i, j) = (next::<usize>(&mut it), next::<usize>(&mut it));
        player.go(&mut board, i, j);
        if player.money < 0 {
            println!("LOSE");
            return;
        }
    }

    println!("{}", if board.price.iter().any(|&x| x > 0) { "LOSE" } else { "WIN" });
}