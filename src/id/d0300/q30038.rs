use std::io::{self, Read, Write};
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

struct Monster {
    x: i32, // x coordinate
    y: i32, // y coordinate
    hp: i32, // health point
    def: i32, // defense power
    exp: i32, // experience
}
impl Monster {
    fn attacked(&mut self, board: &mut Vec<Vec<i32>>, player: &mut Player) -> i32 {
        self.hp -= if player.atk > self.def { player.atk - self.def } else { 0 };
        return if self.hp <= 0 {
            board[self.y as usize][self.x as usize] = 0;
            self.exp
        } else { 0 }
    }
}

struct Player {
    x: i32, // x coordinate
    y: i32, // y coordinate
    atk: i32, // attack power
    rng: i32, // attack range
    spd: i32, // speed
    lvl: i32, // level
    exp: i32, // experience
    erq: i32, // experience required to level up
    act: i32, // action points
    cnt: i32, // dose counter
    ovd: i32, // overdose
    clr: bool, // clear
}
impl Player {
    fn action(&mut self, cost: i32) {
        self.act += cost;
        self.ovd -= cost;
        if self.ovd < 0 { self.ovd = 0; }
    }

    fn gain_exp(&mut self, e: i32) {
        self.exp += e;
        while self.exp >= self.erq {
            self.exp -= self.erq;
            self.atk += self.lvl;
            self.rng += 1;
            self.lvl += 1;
            self.erq += 10;
        }
    }

    fn teleport(&mut self, board: &Vec<Vec<i32>>, dir: u8, n: i32, m: i32) {
        if self.clr { return; }
        match dir {
            0 => { // up
                if self.y - self.spd >= 0 && (board[(self.y - self.spd) as usize][self.x as usize] == 0 || board[(self.y - self.spd) as usize][self.x as usize] == -2) {
                    self.y -= self.spd;
                    self.action(1);
                }
            },
            1 => { // down
                if self.y + self.spd < n && (board[(self.y + self.spd) as usize][self.x as usize] == 0 || board[(self.y + self.spd) as usize][self.x as usize] == -2) {
                    self.y += self.spd;
                    self.action(1);
                }
            },
            2 => { // left
                if self.x - self.spd >= 0 && (board[self.y as usize][(self.x - self.spd) as usize] == 0 || board[self.y as usize][(self.x - self.spd) as usize] == -2) {
                    self.x -= self.spd;
                    self.action(1);
                }
            },
            3 => { // right
                if self.x + self.spd < m && (board[self.y as usize][(self.x + self.spd) as usize] == 0 || board[self.y as usize][(self.x + self.spd) as usize] == -2) {
                    self.x += self.spd;
                    self.action(1);
                }
            },
            _ => {},
        }
    }

    fn wait(&mut self) {
        if self.clr { return; }
        self.action(1);
    }

    fn attack(&mut self, board: &mut Vec<Vec<i32>>, mon: &mut Vec<Monster>, dir: u8) {
        if self.clr { return; }
        if self.ovd > 0 { return; }
        self.action(3);

        let mut prj = (self.x, self.y); // projectile
        let mut amon = vec![]; // attacked monsters
        for _ in 0..self.rng {
            match dir {
                0 => prj.1 -= 1, // up
                1 => prj.1 += 1, // down
                2 => prj.0 -= 1, // left
                3 => prj.0 += 1, // right
                _ => {},
            }
            let obj = board[prj.1 as usize][prj.0 as usize];
            if obj > 0 { amon.push(obj); }
            if obj == -1 { break; }
        }

        let mut e = 0;
        for m in amon {
            e += mon[m as usize].attacked(board, self);
        }
        self.gain_exp(e);
    }

    fn dose_up(&mut self) {
        if self.clr { return; }
        if self.ovd > 0 { return; }
        self.action(2);
        self.spd += 1;
        self.cnt += 1;
        if self.cnt == 5 {
            self.ovd = 10;
            self.cnt = 0;
        }
    }

    fn dose_down(&mut self) {
        if self.clr { return; }
        if self.ovd > 0 { return; }
        self.action(2);
        self.spd -= 1;
        if self.spd < 0 { self.spd = 0; }
        self.cnt += 1;
        if self.cnt == 5 {
            self.ovd = 10;
            self.cnt = 0;
        }
    }

    fn clear(&mut self, board: &Vec<Vec<i32>>) {
        if self.clr { return; }
        if self.ovd > 0 { return; }
        if board[self.y as usize][self.x as usize] == -2 {
            self.clr = true;
        }
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, m) = (next::<i32>(&mut it), next::<i32>(&mut it));
    let mut board = vec![vec![0; m as usize]; n as usize];
    let (mut x, mut y) = (0, 0);
    for i in 0..n {
        let s = next::<String>(&mut it);
        for (j, c) in s.chars().enumerate() {
            match c {
                'p' => { x = j as i32; y = i; },
                'm' => { board[i as usize][j] = 1; },
                '*' => { board[i as usize][j] = -1; },
                'g' => { board[i as usize][j] = -2; },
                _ => {},
            }
        }
    }

    let k = next::<usize>(&mut it);
    let mh = (0..k).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let md = (0..k).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();
    let me = (0..k).map(|_| next::<i32>(&mut it)).collect::<Vec<_>>();

    let mut c = 0;
    let mut mon = vec![Monster { x: 0, y: 0, hp: 0, def: 0, exp: 0 }];
    for i in 0..n {
        for j in 0..m {
            if board[i as usize][j as usize] == 1 {
                let (h, d, e) = (mh[c], md[c], me[c]);
                c += 1;
                board[i as usize][j as usize] = c as i32;
                mon.push(Monster { x: j, y: i, hp: h, def: d, exp: e });
            }
        }
    }

    let mut player = Player { x, y, atk: 5, rng: 1, spd: 1, lvl: 1, exp: 0, erq: 10, act: 0, cnt: 0, ovd: 0, clr: false };
    let a = next::<usize>(&mut it);
    for _ in 0..a {
        let cmd = next::<String>(&mut it);
        let cmd = cmd.as_str();
        match cmd {
            "u" => player.teleport(&board, 0, n, m), // up
            "d" => player.teleport(&board, 1, n, m), // down
            "l" => player.teleport(&board, 2, n, m), // left
            "r" => player.teleport(&board, 3, n, m), // right
            "w" => player.wait(), // wait
            "au" => player.attack(&mut board, &mut mon, 0), // attack up
            "ad" => player.attack(&mut board, &mut mon, 1), // attack down
            "al" => player.attack(&mut board, &mut mon, 2), // attack left
            "ar" => player.attack(&mut board, &mut mon, 3), // attack right
            "du" => player.dose_up(), // dose up
            "dd" => player.dose_down(), // dose down
            "c" => player.clear(&board), // clear
            _ => {},
        }
    }

    writeln!(so, "{} {}", player.lvl, player.exp).ok();
    writeln!(so, "{}", player.act).ok();
    for i in 0..n {
        for j in 0..m {
            if i == player.y && j == player.x {
                write!(so, "p").ok();
                continue;
            }
            match board[i as usize][j as usize] {
                0 => { write!(so, ".").ok(); },
                -1 => { write!(so, "*").ok(); },
                -2 => { write!(so, "g").ok(); },
                _ => { write!(so, "m").ok(); },
            }
        }
        writeln!(so).ok();
    }
    for m in mon {
        if m.hp > 0 {
            write!(so, "{} ", m.hp).ok();
        }
    }
    writeln!(so).ok();
}