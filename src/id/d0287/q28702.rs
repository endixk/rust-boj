use std::io;
pub fn main() {
    let v = io::stdin().lines().take(3).map(|x| x.unwrap()).collect::<Vec<_>>();
    let mut x = 0;
    let a = v[0].parse::<u64>().unwrap_or(0);
    if a > 0 { x = a+3 }
    let a = v[1].parse::<u64>().unwrap_or(0);
    if a > 0 { x = a+2 }
    let a = v[2].parse::<u64>().unwrap_or(0);
    if a > 0 { x = a+1 }

    if x % 15 == 0 { println!("FizzBuzz"); }
    else if x % 3 == 0 { println!("Fizz"); }
    else if x % 5 == 0 { println!("Buzz"); }
    else { println!("{}", x); }
}