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

fn construct(deck: &mut Vec<u8>) -> bool {
    for i in 1..10 {
        if deck[i] > 2 { deck[i] -= 3; }
        if deck[i] > 0 && i+2 < 10 && deck[i+1] >= deck[i] && deck[i+2] >= deck[i] {
            deck[i+1] -= deck[i];
            deck[i+2] -= deck[i];
            deck[i] = 0;
        }
    }
    for i in 11..20 {
        if deck[i] > 2 { deck[i] -= 3; }
        if deck[i] > 0 && i+2 < 20 && deck[i+1] >= deck[i] && deck[i+2] >= deck[i] {
            deck[i+1] -= deck[i];
            deck[i+2] -= deck[i];
            deck[i] = 0;
        }
    }
    for i in 21..30 {
        if deck[i] > 2 { deck[i] -= 3; }
        if deck[i] > 0 && i+2 < 30 && deck[i+1] >= deck[i] && deck[i+2] >= deck[i] {
            deck[i+1] -= deck[i];
            deck[i+2] -= deck[i];
            deck[i] = 0;
        }
    }
    for i in 31..38 {
        if deck[i] > 2 { deck[i] -= 3; }
    }
    deck.iter().all(|&x| x == 0)
}
fn valid(deck: &Vec<u8>) -> bool {
    if deck.iter().filter(|&&x| x == 2).count() == 7 { return true; }
    if deck[1] > 0 && deck[9] > 0
        && deck[11] > 0 && deck[19] > 0
        && deck[21] > 0 && deck[29] > 0
        && deck[31] > 0 && deck[32] > 0
        && deck[33] > 0 && deck[34] > 0
        && deck[35] > 0 && deck[36] > 0 && deck[37] > 0
        && deck[1] + deck[9] + deck[11] + deck[19] + deck[21] + deck[29] + deck[31] + deck[32] + deck[33] + deck[34] + deck[35] + deck[36] + deck[37] == 14 {
        return true;
    }
    for i in 1..38 {
        if deck[i] > 1 {
            let mut c = deck.clone();
            c[i] -= 2;
            if construct(&mut c) { return true; }
        }
    }
    false
}

fn code(card: &[u8]) -> usize {
    match card[0] {
        b'e' => 31,
        b's' => 32,
        b'w' => 33,
        b'n' => 34,
        b'h' => 35,
        b'b' => 36,
        b'j' => 37,
        x => {
            let off = x + 1 - b'1';
            let ret = match card[1] {
                b's' => off,
                b't' => off + 10,
                _ => off + 20,
            };
            ret as usize
        }
    }
}
const MAJHONG: [&str; 38] = [
    "", "1s", "2s", "3s", "4s", "5s", "6s", "7s", "8s", "9s",
    "", "1t", "2t", "3t", "4t", "5t", "6t", "7t", "8t", "9t",
    "", "1m", "2m", "3m", "4m", "5m", "6m", "7m", "8m", "9m",
    "", "e", "s", "w", "n", "h", "b", "j"
];
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut deck = vec![0; 38];
    for _ in 0..13 {
        let card = next::<String>(&mut it);
        deck[code(card.as_bytes())] += 1;
    }

    let mut ans = Vec::new();
    for i in 1..10 {
        if deck[i] < 4 {
            deck[i] += 1;
            if valid(&deck) { ans.push(MAJHONG[i].to_string()); }
            deck[i] -= 1;
        }
    }
    for i in 11..20 {
        if deck[i] < 4 {
            deck[i] += 1;
            if valid(&deck) { ans.push(MAJHONG[i].to_string()); }
            deck[i] -= 1;
        }
    }
    for i in 21..30 {
        if deck[i] < 4 {
            deck[i] += 1;
            if valid(&deck) { ans.push(MAJHONG[i].to_string()); }
            deck[i] -= 1;
        }
    }
    for i in 31..38 {
        if deck[i] < 4 {
            deck[i] += 1;
            if valid(&deck) { ans.push(MAJHONG[i].to_string()); }
            deck[i] -= 1;
        }
    }

    if ans.is_empty() {
        println!("no tenpai");
    } else {
        ans.sort_unstable();
        println!("tenpai");
        println!("{}", ans.len());
        println!("{}", ans.join(" "));
    }
}