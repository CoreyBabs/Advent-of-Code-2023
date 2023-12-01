fn main() {
    let input = include_str!("../input.txt");
    let input = input.split("\n").collect::<Vec<&str>>();
    part1(input);
    // println!("{}", input[0]);
}

fn first_and_last_to_number(line: &Vec<char>) -> u32
{
    if line.len() == 0 {
        return 0;
    }

    let mut amount = "".to_string();
    amount.push(*line.first().unwrap());
    amount.push(*line.last().unwrap());
    amount.parse::<u32>().unwrap()
}

fn part1(input: Vec<&str>) {
    let digits: Vec<Vec<char>> = input.iter()
    .map(|l| l.chars().filter(|c| c.is_digit(10)).collect())
    .collect();

    let sum: u32 = digits.iter()
        .map(|d| first_and_last_to_number(&d))
        .sum();

    println!("{}", sum);
}
