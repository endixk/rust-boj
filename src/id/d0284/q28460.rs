// TODO FAILED

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

// S2..SA = 0..12
// D2..DA = 13..25
// H2..HA = 26..38
// C2..CA = 39..51
// JC, JB = 52, 53
fn map(card: &[u8]) -> usize {
    let (c, n) = (card[0], card[1]);
    let c = match c {
        b'S' => 0,
        b'D' => 13,
        b'H' => 26,
        b'C' => 39,
        b'J' => 52,
        _ => {
            println!("Rule Violation");
            std::process::exit(0);
        }
    };
    if c == 52 {
        return if n == b'C' { 52 }
        else if n == b'B' { 53 }
        else {
            println!("Rule Violation");
            std::process::exit(0);
        };
    }
    let n = match n {
        b'2'..=b'9' => n - b'2',
        b'1' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => {
            println!("Rule Violation");
            std::process::exit(0);
        }
    };
    (c + n) as usize
}
fn election_score(rule: u8, obj: u8) -> u8 {
    let obj = if rule == 0 { obj + 1 } else { obj };
    obj * 5 + (4 - rule)
}
fn card_score(card: usize, game: &Game, round: &Round) -> usize {
    let mut class = 0;
    if game.rule == 0 {
        if card == game.mighty {}
        else if card == 52 { // colored joker
            if round.number == 0 || round.number == 9 { class = 5; }
            else if round.jccall { class = 5; }
            else if round.color == 1 || round.color == 2 || round.color == 5 { class = 1; }
            else { class = 3; }
        }
        else if card == 53 { // black joker
            if round.number == 0 || round.number == 9 { class = 5; }
            else if round.jbcall { class = 5; }
            else if round.color == 0 || round.color == 3 || round.color == 4 { class = 1; }
            else { class = 3; }
        }
        else {
            if round.color == 4 {
                if card < 13 || card > 38 { class = 2; }
                else { class = 4; }
            } else if round.color == 5 {
                if card > 12 && card < 39 { class = 2; }
                else { class = 4; }
            } else if card as u8 / 13 == round.color { class = 2; }
            else { class = 4; }
        }
    } else {
        if card == game.mighty {}
        else if card == 52 { // colored joker
            if round.number == 0 || round.number == 9 { class = 6; }
            else if round.jccall { class = 6 }
            else if game.rule == 2 || game.rule == 3 { class = 1; }
            else { class = 3; }
        }
        else if card == 53 { // black joker
            if round.number == 0 || round.number == 9 { class = 6; }
            else if round.jbcall { class = 6 }
            else if game.rule == 1 || game.rule == 4 { class = 1; }
            else { class = 3; }
        }
        else if card as u8 / 13 + 1 == game.rule { class = 2; }
        else {
            if round.color == 4 {
                if card < 13 || card > 38 { class = 4; }
                else { class = 5; }
            } else if round.color == 5 {
                if card > 12 && card < 39 { class = 4; }
                else { class = 5; }
            } else if card as u8 / 13 == round.color { class = 4; }
            else { class = 5; }
        }
    }
    class * 100 + card / 13 * 20 + (12 - card % 13)
}

struct Game {
    rule: u8, // ruling color of the game (0: X, 1: S, 2: D, 3: H, 4: C)
    obj: u8, // objective score of the game
    mighty: usize, // mighty card
    jcbust: usize, // colored joker buster
    jbbust: usize, // black joker buster
}
impl Game {
    fn new(rule: u8, obj: u8) -> Self {
        let mighty = if rule == 1 { 25 } else { 12 };
        let jcbust = if rule == 3 { 14 } else { 27 };
        let jbbust = if rule == 4 { 1 } else { 40 };
        Game { rule, obj, mighty, jcbust, jbbust }
    }
}
struct Round {
    number: usize, // number of the round
    color: u8, // accepted color of the round (0: S, 1: D, 2: H, 3: C, 4: S/C, 5: D/H)
    jccall: bool, // colored joker is called
    jbcall: bool, // black joker is called
}
struct Player {
    deck: [bool; 54],
    gov: bool,
    score: u8,
}
impl Player {
    fn deal_miss(&self) -> bool {
        let mut score = 0;
        for i in 0..54 {
            if self.deck[i] {
                if i > 51 { score -= 1; }
                else if i == 12 { score -= 2; }
                else if i % 13 > 7 { score += 2; }
            }
        }
        score <= 2
    }
    fn play(&self, card: usize, game: &Game, round: &Round) -> bool {
        if !self.deck[card] { return false; }
        // card is the mighty
        if card == game.mighty { return true; }
        // colored joker is called and player has colored joker
        if round.jccall && self.deck[52] {
            return card == 52;
        }
        // black joker is called and player has black joker
        if round.jbcall && self.deck[53] {
            return card == 53;
        }
        // card is a joker
        if card == 52 || card == 53 { return true; }
        // player has the accepted color
        let mut flag = false;
        if round.color == 4 {
            for i in 0..13 { flag |= self.deck[i] }
            for i in 39..52 { flag |= self.deck[i] }
            if flag && (card > 12 && card < 39) { return false; }
        } else if round.color == 5 {
            for i in 13..39 { flag |= self.deck[i] }
            if flag && (card < 13 || card > 38) { return false; }
        } else {
            for i in (round.color as usize * 13)..((round.color as usize + 1) * 13) {
                flag |= self.deck[i];
            }
            if flag && (card < round.color as usize * 13 || card >= (round.color as usize + 1) * 13) {
                return false;
            }
        }
        true
    }
}
pub fn main() {
    let mut si = io::BufReader::new(io::stdin().lock());
    let s = read(&mut si);
    let mut it = s.split_ascii_whitespace();

    let mut players = Vec::new();
    for _ in 0..5 {
        players.push(Player { deck: [false; 54], gov: false, score: 0 });
    }
    let mut field_deck = [false; 54];

    // Deal
    for i in 0..5 {
        let player = &mut players[i];
        for _ in 0..10 {
            let card = next::<String>(&mut it);
            let card = map(card.as_bytes());
            if player.deck[card] {
                println!("Rule Violation");
                return;
            }
            player.deck[card] = true;
            field_deck[card] = true;
        }
    }
    for player in &players {
        if player.deal_miss() {
            println!("Deal Mistake");
            return;
        }
    }

    // Election
    let mut rules = [(0, 0); 5];
    for i in 0..5 {
        let e = next::<String>(&mut it);
        let e = e.as_bytes();
        if e[0] == b'N' { continue; }
        let rule = match e[0] {
            b'X' => 0,
            b'S' => 1,
            b'D' => 2,
            b'H' => 3,
            b'C' => 4,
            _ => {
                println!("Rule Violation");
                return;
            }
        };
        let obj = String::from_utf8(e[1..].to_vec()).unwrap().parse::<u8>().unwrap();
        rules[i] = (rule, obj);
    }
    let (mut pres, mut rule, mut obj) = (0, 0, 0);
    for i in 0..5 {
        if election_score(rules[i].0, rules[i].1) > election_score(rule, obj) {
            pres = i;
            (rule, obj) = rules[i];
        }
    }

    // Presidential acts
    players[pres].gov = true;
    let game = Game::new(rule, obj);
    for i in 0..54 {
        if !field_deck[i] {
            players[pres].deck[i] = true;
        }
    }
    for _ in 0..4 {
        let disc = next::<String>(&mut it);
        let disc = map(disc.as_bytes());
        if players[pres].deck[disc] {
            players[pres].deck[disc] = false;
            if disc % 13 > 7 { players[pres].score += 1; }
        } else {
            println!("Rule Violation");
            return;
        }
    }
    let call = next::<String>(&mut it);
    let call = map(call.as_bytes());
    for player in &mut players {
        if player.deck[call] {
            player.gov = true;
        }
    }

    // Rounds
    for i in 0..10 {
        let mut cards = (0..5).map(|_| next::<String>(&mut it)).collect::<Vec<_>>();
        let mut score = [0; 5];
        let first = cards[pres].as_bytes();
        if first[0] == b'J' {
            let mut color = 0;
            if first.len() == 3 {
                if first[1] == b'S' {
                    color = 4;
                    if !players[pres].deck[53] {
                        println!("Rule Violation");
                        return;
                    }
                    players[pres].deck[53] = false;
                    cards[pres] = String::from_utf8(vec![b'J', b'B']).unwrap();
                } else {
                    color = 5;
                    if !players[pres].deck[52] {
                        println!("Rule Violation");
                        return;
                    }
                    players[pres].deck[52] = false;
                    cards[pres] = String::from_utf8(vec![b'J', b'C']).unwrap();
                }
            } else {
                if first[1] == b'S' {
                    // color = 0;
                    if !players[pres].deck[53] {
                        println!("Rule Violation");
                        return;
                    }
                    players[pres].deck[53] = false;
                    cards[pres] = String::from_utf8(vec![b'J', b'B']).unwrap();
                } else if first[1] == b'D' {
                    color = 1;
                    if !players[pres].deck[52] {
                        println!("Rule Violation");
                        return;
                    }
                    players[pres].deck[52] = false;
                    cards[pres] = String::from_utf8(vec![b'J', b'C']).unwrap();
                } else if first[1] == b'H' {
                    color = 2;
                    if !players[pres].deck[52] {
                        println!("Rule Violation");
                        return;
                    }
                    players[pres].deck[52] = false;
                    cards[pres] = String::from_utf8(vec![b'J', b'C']).unwrap();
                } else {
                    color = 3;
                    if !players[pres].deck[53] {
                        println!("Rule Violation");
                        return;
                    }
                    players[pres].deck[53] = false;
                    cards[pres] = String::from_utf8(vec![b'J', b'B']).unwrap();
                }
            }

            let round = Round { number: i, color, jccall: false, jbcall: false };
            for i in 0..5 {
                let card = map(cards[i].as_bytes());
                score[i] = card_score(card, &game, &round);
                if i == pres { continue; }
                if players[i].play(card, &game, &round) {
                    players[i].deck[card] = false;
                } else {
                    println!("Rule Violation");
                    return;
                }
            }
        } else {
            let cards = cards.iter().map(|c| map(c.as_bytes())).collect::<Vec<_>>();
            let first = cards[pres];
            if !players[pres].deck[first] {
                println!("Rule Violation");
                return;
            }
            if i == 0 && game.rule > 0 {
                let color = (game.rule - 1) as usize;
                let mut cnt = 0;
                for i in 0..13 {
                    if players[pres].deck[13 * color + i] { cnt += 1; }
                }
                if cnt < 10 && first / 13 == color {
                    println!("Rule Violation");
                    return;
                }
            }
            players[pres].deck[first] = false;

            let round = Round { number: i, color: (first as u8) / 13, jccall: first == game.jcbust, jbcall: first == game.jbbust };
            for i in 0..5 {
                score[i] = card_score(cards[i], &game, &round);
                if i == pres { continue; }
                if players[i].play(cards[i], &game, &round) {
                    players[i].deck[cards[i]] = false;
                } else {
                    println!("Rule Violation");
                    return;
                }
            }
        }

        let mut min = 9999999;
        for i in 0..5 {
            if score[i] < min {
                min = score[i];
                pres = i;
            }
        }
        for card in cards.iter().map(|c| map(c.as_bytes())) {
            if card % 13 > 7 {
                players[pres].score += 1;
            }
        }
    }

    let mut score = 0;
    for player in &players {
        if player.gov { score += player.score; }
    }
    if score >= game.obj {
        println!("{} Government Party", score as i32 - game.obj as i32);
    } else {
        println!("{} Opposition Party", score as i32 - game.obj as i32);
    }
}