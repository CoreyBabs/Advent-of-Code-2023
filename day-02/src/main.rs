use core::panic;
use std::cmp;

fn main() {
    let input = include_str!("../input.txt");
    let mut input = input.split("\n").collect::<Vec<&str>>();
    input.pop();
    part1(&input);
}

fn part1(input: &Vec<&str>) {
    let sum: u32 = input.iter()
        .map(|l| Game::from_line(l))
        .filter(|g| g.valid)
        .map(|g| g.id)
        .sum();

    println!("{}", sum);
}


#[derive(Debug)]
struct Game {
    id: u32,
    valid: bool,
    red: u32,
    green: u32,
    blue: u32
}

impl Game {
    fn from_line(line: &str) -> Game {
        let split: Vec<&str> = line.split(":").collect();
        let (r, g, b) = Self::get_max_rgb(split[1]);
        Game { id: Self::get_id(split[0]), valid: Self::check_valid(r, g, b), blue: b, red: r, green: g }
    }

    fn get_id(game: &str) -> u32 {
        let id = game.split(" ").collect::<Vec<&str>>()[1];
        id.parse::<u32>().unwrap()
    }

    fn get_rgb(rounds: &str) -> (u32, u32, u32) {
        let rounds: Vec<&str> = rounds.split(";").collect();
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;
        for round in rounds {
            let cubes: Vec<&str> = round.split(", ").collect();
            cubes.iter()
                .for_each(|c| {
                    let count: Vec<&str> = c.trim().split(" ").collect();
                    match count[1] {
                        "red" => r += count[0].parse::<u32>().unwrap(),
                        "green" => g += count[0].parse::<u32>().unwrap(),
                        "blue" => b += count[0].parse::<u32>().unwrap(),
                        _ => panic!("Invalid color")
                    }
                });
        }

        (r,g,b)
    } 

    fn get_max_rgb(rounds: &str) -> (u32, u32, u32) {
        let rounds: Vec<&str> = rounds.split(";").collect();
        let mut r: u32 = 0;
        let mut g: u32 = 0;
        let mut b: u32 = 0;
        for round in rounds {
            let cubes: Vec<&str> = round.split(", ").collect();
            cubes.iter()
                .for_each(|c| {
                    let count: Vec<&str> = c.trim().split(" ").collect();
                    match count[1] {
                        "red" => r = cmp::max(r, count[0].parse::<u32>().unwrap()),
                        "green" => g = cmp::max(g, count[0].parse::<u32>().unwrap()),
                        "blue" => b = cmp::max(b , count[0].parse::<u32>().unwrap()),
                        _ => panic!("Invalid color")
                    }
                });
        }

        (r,g,b)
    } 
    fn check_valid(red: u32, green: u32, blue: u32) -> bool {
        let max_r = 12;
        let max_g = 13;
        let max_b = 14;
        red <= max_r && green <= max_g && blue <= max_b
    }
}
