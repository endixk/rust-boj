use std::io::BufRead;
pub fn main() {
    let v = std::io::stdin().lock().lines().next().unwrap().unwrap()
        .split_ascii_whitespace().map(|s| s.to_string()).collect::<Vec<_>>();
    println!("{}", u32::from_str_radix(v[0].as_str(), v[1].parse::<u32>().unwrap()).unwrap());
}
