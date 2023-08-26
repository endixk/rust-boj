use std::io::BufRead;
fn main(){
    let m = [3,3,3,4,4,4,5,5,5,6,6,6,7,7,7,8,8,8,8,9,9,9,10,10,10,10];
    println!("{}",std::io::stdin().lock().lines().next().unwrap().unwrap().chars().fold(0,|a,c|a+m[c as usize-65]));
}