fn main() {
    let input = include_str!("../input.txt");
    let mut input = input.split("\n").collect::<Vec<&str>>();
    input.pop();
    part1(&input);
}

fn line_to_vec(line: &str) -> Vec<&str> {
    let split: Vec<&str> = line.split(":").collect();

    split[1].split(" | ").collect()
}

fn vec_to_score(numbers: &Vec<&str>) -> usize {
    let winning: Vec<&str> = numbers[0].split(" ").collect();
    let nums: Vec<&str> = numbers[1].split(" ").collect();
    
    let amount: usize = nums.iter().filter(|n| !n.is_empty() && winning.contains(n)).count();

    if amount == 0 {
        return 0;
    }

    2_usize.pow(amount as u32 - 1) 
}

fn part1(input: &Vec<&str>) {
    let sum: usize = input.iter()
        .map(|l| line_to_vec(l))
        .map(|l| vec_to_score(&l))
        .sum();

    println!("{}", sum);
}
