// BOJ 19948 [Bard]
// Supported by GitHub Copilot

use std::io::{self, BufRead};
fn read<T>(si: &mut T) -> String where T: BufRead {
    let mut s = String::new();
    si.read_line(&mut s).unwrap();
    s.trim().to_string()
}

pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());

    let poem = read(&mut si);
    let mut s = read(&mut si).parse::<i32>().unwrap();
    let mut v = read(&mut si).split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let title = poem.split_whitespace().map(|s| s[0..1].to_string().to_uppercase()).collect::<Vec<String>>().join("");
    let mut td = title.chars().collect::<Vec<char>>();
    td.dedup();
    td.iter().collect::<String>().to_lowercase().chars().map(|c| c as usize - 'a' as usize).for_each(|i| {
        v[i] -= 1;
        if v[i] < 0 {
            println!("-1");
            std::process::exit(0);
        }
    });

    let mut poem = poem.chars().collect::<Vec<char>>();
    poem.dedup();
    poem.iter().collect::<String>().to_lowercase().chars().for_each(|c| {
        if c == ' ' {
            s -= 1;
            if s < 0 {
                println!("-1");
                std::process::exit(0);
            }
        } else {
            v[c as usize - 'a' as usize] -= 1;
            if v[c as usize - 'a' as usize] < 0 {
                println!("-1");
                std::process::exit(0);
            }
        }
    });

    println!("{}", title);
}
