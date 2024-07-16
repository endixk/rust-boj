// BOJ 6676 [Inglish-Number Translator]
use std::io::prelude::*;
pub fn main() {
    let mut it = std::io::stdin().lock().lines();
    while let Some(Ok(s)) = it.next() {
        if s.len() == 0 { break; }
        let mut x = 0;
        let mut b = 0;
        for w in s.split_ascii_whitespace() {
            match w {
                "negative" => print!("-"),
                "zero" => continue,
                "one" => b += 1,
                "two" => b += 2,
                "three" => b += 3,
                "four" => b += 4,
                "five" => b += 5,
                "six" => b += 6,
                "seven" => b += 7,
                "eight" => b += 8,
                "nine" => b += 9,
                "ten" => b += 10,
                "eleven" => b += 11,
                "twelve" => b += 12,
                "thirteen" => b += 13,
                "fourteen" => b += 14,
                "fifteen" => b += 15,
                "sixteen" => b += 16,
                "seventeen" => b += 17,
                "eighteen" => b += 18,
                "nineteen" => b += 19,
                "twenty" => b += 20,
                "thirty" => b += 30,
                "forty" => b += 40,
                "fifty" => b += 50,
                "sixty" => b += 60,
                "seventy" => b += 70,
                "eighty" => b += 80,
                "ninety" => b += 90,
                "hundred" => b *= 100,
                "thousand" => { x += b * 1000; b = 0; },
                "million" => { x += b * 1000000; b = 0; },
                _ => panic!()
            }
        }
        println!("{}", x + b);
    }
}