fn main() {
    let input = include_str!("../input.txt");
    let input = input.split("\n").collect::<Vec<&str>>();
    part1(input);
    // println!("{}", input[0]);
}

fn part1(input: Vec<&str>) {
    let digits: Vec<Vec<char>> = input.iter()
    .map(|l| l.chars().filter(|c| c.is_digit(10)).collect())
    .collect();
    let mut sum = 0;
    for digit in digits {
        if digit.len() == 0 {
            continue;
        }

        let mut amount = "".to_string();
        amount.push(*digit.first().unwrap());
        amount.push(*digit.last().unwrap());

        sum += amount.parse::<u32>().unwrap();
    }

    println!("{}", sum);
}
