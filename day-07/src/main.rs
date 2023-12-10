use std::{collections::HashMap, cmp::Ordering};

fn main() {
    let input = include_str!("../input.txt");
    let mut input: Vec<&str> = input.split("\n").collect();
    input.pop();
    part1(&input);
}

fn part1(input: &Vec<&str>) {
    let mut hands: Vec<Hand> = input.iter().map(|l| {
        let mut hand = Hand::new(l);
        hand.calculate_strength();
        hand
    })
        .collect();
    hands.sort_by(compare_hands);

    let mut sum = 0; 
    for (i, hand) in hands.iter().enumerate() {
        sum += hand.bid as usize * (i + 1);
    }

    println!("{:?}", sum);
}

fn compare_hands(l: &Hand, r: &Hand) -> Ordering {
    if l.stength != r.stength {
        return l.stength.cmp(&r.stength);
    }

    let mut idx = 0;
    let mut lc = l.hand[idx];
    let mut rc = r.hand[idx];
    while lc == rc {
        idx += 1;
        lc = l.hand[idx];
        rc = r.hand[idx];
    }

    if lc.is_alphabetic() && rc.is_alphabetic() {
        return royal_to_number(&lc).cmp(&royal_to_number(&rc));
    }

    lc.cmp(&rc)
}

fn royal_to_number(royal: &char) -> &str {
    match royal {
        'A' => "14",
        'K' => "13",
        'Q' => "12",
        'J' => "11",
        'T' => "10",
        _ => panic!("Invalid royaly card"),
    }
}


#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    hand: Vec<char>,
    bid: u32,
    rank: u32,
    stength: u8
}

impl Hand {
    fn new(line: &str) -> Hand {
        let line: Vec<&str> = line.split(" ").collect();
        let hand: Vec<char> = line[0].chars().collect();
        let bid: u32 = line[1].parse().unwrap();
        Hand { hand, bid, rank: 0, stength: 0 }
    } 

    fn calculate_winnings(&self) -> u32 {
        self.bid * self.rank 
    }

    fn calculate_strength(&mut self) {
        let mut map: HashMap<char, u8> = HashMap::new();

        for card in &self.hand {
            map.entry(*card)
                .and_modify(|e| {*e += 1})
                .or_insert(1);
        }

        match map.keys().len() {
            1 => self.stength = 6,
            2 => {
                let count = map.get(map.keys().nth(0).unwrap()).unwrap();
                if count == &1 || count == &4 {
                    self.stength = 5;
                }
                else {
                    self.stength = 4;
                }
            }
            3 => {
                if map.values().collect::<Vec<&u8>>().contains(&&3) {
                    self.stength = 3;
                }
                else {
                    self.stength = 2;
                }
            },
            4 => self.stength = 1,
            5 => self.stength = 0,
            _ => panic!("Invalid number of cards in hand")
            
        }
    }
}
