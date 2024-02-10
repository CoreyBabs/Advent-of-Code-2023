use std::{collections::HashMap, cmp::Ordering};

fn main() {
    let input = include_str!("../input.txt");
    let mut input: Vec<&str> = input.split("\n").collect();
    input.pop();
    part1(&input);
    part2(&input);
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

fn part2(input: &Vec<&str>) {
    let mut hands: Vec<Hand> = input.iter().map(|l| {
        let mut hand = Hand::new(l);
        let original = hand.hand.clone();
        hand.hand = Hand::generate_strongest_hand(&hand.hand);
        hand.calculate_strength();
        hand.hand = original;
        hand
    })
        .collect();
    hands.sort_by(compare_hands_with_joker);

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
    while lc == rc && idx < l.hand.len() - 1 {
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

fn compare_hands_with_joker(l: &Hand, r: &Hand) -> Ordering {
    if l.stength != r.stength {
        return l.stength.cmp(&r.stength);
    }

    let mut idx = 0;
    let mut lc = l.hand[idx];
    let mut rc = r.hand[idx];
    while lc == rc && idx < l.hand.len() - 1 {
        idx += 1;
        lc = l.hand[idx];
        rc = r.hand[idx];
    }

    if lc.is_alphabetic() && rc.is_alphabetic() {
        return royal_to_number_with_joker(&lc).cmp(&royal_to_number_with_joker(&rc));
    }

    let ls: usize;
    let rs: usize;
    
    if lc.is_alphabetic() {
        ls = royal_to_number_with_joker(&lc).parse().unwrap();
    }
    else {
        ls = lc.to_digit(10).unwrap() as usize;
    }

    if rc.is_alphabetic() {
        rs = royal_to_number_with_joker(&rc).parse().unwrap();
    }
    else {
        rs = rc.to_digit(10).unwrap() as usize;
    }

    ls.cmp(&rs)
}

fn royal_to_number_with_joker(royal: &char) -> &str {
    match royal {
        'A' => "14",
        'K' => "13",
        'Q' => "12",
        'J' => "0",
        'T' => "10",
        _ => panic!("Invalid royaly card"),
    }
}


fn generate_possible_hands(hand: &Vec<char>, index: usize) -> Vec<Vec<char>> {
    let possible = vec![
        '1',
        '2',
        '3',
        '4',
        '5',
        '6',
        '7',
        '8',
        '9',
        'T',
        'J',
        'Q',
        'K',
        'A'
    ];

    let mut hands = vec![];

    for p in possible {
        let mut new = hand.clone();
        new[index] = p;
       hands.push(new); 
    }

    hands
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

    fn generate_strongest_hand(hand: &Vec<char>) -> Vec<char> {
        if !hand.contains(&'J') {
            return hand.clone();
        }


        let mut indexes = vec![];
        for (i, card) in hand.iter().enumerate() {
            if card == &'J' {
                indexes.push(i);
            }
        }

        let mut all_hands = generate_possible_hands(&hand, indexes[0]);
        for (i, idx) in indexes.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let mut new_hands = vec![];
            for hand in all_hands.iter() {
                new_hands.append(&mut generate_possible_hands(&hand, *idx));
            }

            all_hands = new_hands;
        }

        let mut all_hands: Vec<Hand> = all_hands.iter()
            .map(|h| {
                let mut hand = Hand { hand: h.to_vec(), bid: 0, rank: 0, stength: 0 };
                hand.calculate_strength();
                hand
            })
            .collect();

        
        all_hands.sort_by(compare_hands_with_joker);

        all_hands.last().unwrap().hand.clone()
    }
}
