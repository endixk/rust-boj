// BOJ 15898 [Alchemist]
// Supported by GitHub Copilot

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

#[derive(Clone)]
struct Recipe {
    q: [[i8; 4]; 4],
    c: [[char; 4]; 4],
}
impl Recipe {
    fn rotate(&self) -> Recipe {
        let mut q = [[0; 4]; 4];
        let mut c = [['W'; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                q[i][j] = self.q[3 - j][i];
                c[i][j] = self.c[3 - j][i];
            }
        }
        Recipe { q, c }
    }
}

#[derive(Default)]
struct Grid {
    q: [[i8; 5]; 5],
    c: [[char; 5]; 5],
}
impl Grid {
    fn put(&mut self, r: &Recipe, i: usize, j: usize) {
        for k in 0..4 {
            for l in 0..4 {
                self.q[i + k][j + l] += r.q[k][l];
                if self.q[i + k][j + l] < 0 { self.q[i + k][j + l] = 0; }
                if self.q[i + k][j + l] > 9 { self.q[i + k][j + l] = 9; }
                if r.c[k][l] != 'W' { self.c[i + k][j + l] = r.c[k][l]; }
            }
        }
    }
    fn score(&self) -> i32 {
        let mut s = 0;
        for i in 0..5 {
            for j in 0..5 {
                s += self.q[i][j] as i32 * match self.c[i][j] {
                    'R' => 7,
                    'B' => 5,
                    'G' => 3,
                    'Y' => 2,
                    _ => 0,
                };
            }
        }
        s
    }
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let n = next::<usize>(&mut it);
    let mut recipes = Vec::new();
    for _ in 0..n {
        let mut q = [[0; 4]; 4];
        let mut c = [['W'; 4]; 4];
        for i in 0..4 { for j in 0..4 {
            q[i][j] = next::<i8>(&mut it);
        }}
        for i in 0..4 { for j in 0..4 {
            c[i][j] = next::<char>(&mut it);
        }}

        let mut rs = Vec::new();
        let r = Recipe { q, c };
        for i in 0..4 {
            let mut recipe = r.clone();
            for _ in 0..i { recipe = recipe.rotate(); }
            rs.push((recipe.clone(), 0, 0));
            rs.push((recipe.clone(), 0, 1));
            rs.push((recipe.clone(), 1, 0));
            rs.push((recipe.clone(), 1, 1));
        }
        recipes.push(rs);
    }

    let mut ans = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i == j || i == k || j == k { continue; }
                for (ir, ii, ij) in &recipes[i] {
                    for (jr, ji, jj) in &recipes[j] {
                        for (kr, ki, kj) in &recipes[k] {
                            let mut grid = Grid::default();
                            grid.put(ir, *ii, *ij);
                            grid.put(jr, *ji, *jj);
                            grid.put(kr, *ki, *kj);
                            ans = ans.max(grid.score());
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
