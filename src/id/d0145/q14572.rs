// BOJ 14572 [Study Group]
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
struct Student {
    skill: usize,
    topics: Vec<usize>,
}

fn add(tvec: &mut Vec<usize>, stu: &Student) {
    for &t in &stu.topics {
        tvec[t] += 1;
    }
}
fn del(tvec: &mut Vec<usize>, stu: &Student) {
    for &t in &stu.topics {
        tvec[t] -= 1;
    }
}
fn score(tvec: &Vec<usize>, n: usize) -> usize {
    tvec.iter().filter(|&&t| t > 0 && t < n).count() * n
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let (n, k, d) = (next::<usize>(&mut it), next::<usize>(&mut it), next::<usize>(&mut it));
    let mut group = Vec::new();
    for _ in 0..n {
        let (t, skill) = (next::<usize>(&mut it), next::<usize>(&mut it));
        let topics = (0..t).map(|_| next::<usize>(&mut it)).collect::<Vec<_>>();
        group.push(Student { skill, topics });
    }
    group.sort_unstable_by_key(|s| s.skill);

    let mut tvec = vec![0; k + 1];
    let mut ans = 0;
    let (mut l, mut r) = (0, 0);
    while r < n {
        while r < n && group[r].skill - group[l].skill <= d {
            add(&mut tvec, &group[r]);
            r += 1;
        }
        if group[r - 1].skill - group[l].skill <= d {
            ans = ans.max(score(&tvec, r - l));
        }
        del(&mut tvec, &group[l]);
        l += 1;
    }
    println!("{}", ans);
}
