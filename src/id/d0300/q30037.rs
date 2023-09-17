use std::io::{self, BufRead, Write};
fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn kof(words: Vec<String>) -> Vec<String> {
    let mut ret = Vec::new();
    let n = words.len();
    let mut i = 0;
    while i < n-1 {
        if ret.is_empty() {
            ret.push(words[i].clone());
            i += 1; continue;
        }

        let w = ret.pop().unwrap();
        if w.ends_with("!") || w.ends_with("?") || w.ends_with(",") || w.ends_with(".") {
            ret.push(w);
            ret.push(words[i].clone());
            i += 1; continue;
        }

        if words[i] != "of" {
            ret.push(w);
            ret.push(words[i].clone());
            i += 1; continue;
        }

        let mut cap = w.clone().chars().collect::<Vec<_>>();
        if cap[0].is_lowercase() {
            cap[0] = cap[0].to_uppercase().nth(0).unwrap();
        }
        let cap = cap.into_iter().collect::<String>();
        if words[i+1] == "Korea" {
            ret.push(format!("K-{}", cap));
            i += 2; continue;
        }
        if words[i+1] == "Korea!" || words[i+1] == "Korea?" || words[i+1] == "Korea," || words[i+1] == "Korea." {
            ret.push(format!("K-{}{}", cap, words[i+1].chars().nth(5).unwrap()));
            i += 2; continue;
        }

        ret.push(w);
        ret.push(words[i].clone());
        i += 1;
    }
    while i < n {
        ret.push(words[i].clone());
        i += 1;
    }
    ret
}

fn kor(words: Vec<String>) -> Vec<String> {
    let mut ret = Vec::new();
    for i in (0..words.len()).rev() {
        if ret.is_empty() {
            ret.push(words[i].clone());
            continue;
        }

        let w = ret.pop().unwrap();
        let mut cap = w.clone().chars().collect::<Vec<_>>();
        if cap[0].is_lowercase() {
            cap[0] = cap[0].to_uppercase().nth(0).unwrap();
        }
        let cap = cap.into_iter().collect::<String>();
        if words[i] == "Korea" {
            ret.push(format!("K-{}", cap));
            continue;
        }

        ret.push(w);
        ret.push(words[i].clone());
    }
    ret.reverse();
    ret
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let mut so = io::BufWriter::new(io::stdout().lock());
    let s = read(&mut si);

    let n = s.parse::<usize>().unwrap();
    for _ in 0..n {
        let s = read(&mut si);
        let s = s.split_whitespace().map(|x| x.to_string()).collect::<Vec<String>>();
        let s = kof(s);
        let s = kor(s);
        s.iter().for_each(|x| write!(so, "{} ", x).unwrap());
        writeln!(so, "").unwrap();
    }
}