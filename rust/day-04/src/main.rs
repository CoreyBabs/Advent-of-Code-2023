fn main() {
    let input = include_str!("../input.txt");
    let mut input = input.split("\n").collect::<Vec<&str>>();
    input.pop();
    part1(&input);
    part2(&input);
}

fn line_to_vec(line: &str) -> Vec<&str> {
    let split: Vec<&str> = line.split(":").collect();

    split[1].split(" | ").collect()
}

fn vec_to_count(numbers: &Vec<&str>) -> usize {
    let winning: Vec<&str> = numbers[0].split(" ").collect();
    let nums: Vec<&str> = numbers[1].split(" ").collect();
    
    nums.iter().filter(|n| !n.is_empty() && winning.contains(n)).count()
}

fn count_to_score(amount: usize) -> usize {
    if amount == 0 {
        return 0;
    }

    2_usize.pow(amount as u32 - 1) 
}

fn part1(input: &Vec<&str>) {
    let sum: usize = input.iter()
        .map(|l| line_to_vec(l))
        .map(|l| count_to_score(vec_to_count(&l)))
        .sum();

    println!("{}", sum);
}

fn part2(input: &Vec<&str>) {
    let mut cards: Vec<u32> = vec![1; input.len()];

    let amounts: Vec<usize> = input.iter()
        .map(|l| line_to_vec(l))
        .map(|l| vec_to_count(&l)).collect();

    for (i, amount) in amounts.iter().enumerate() {
        if amount == &0 {
            continue;
        }

        let before = cards.clone();
        for j in i+1..i+amount+1 {
            cards[j] += before[i];
        } 
    }

    let sum: u32 = cards.iter().sum();
    println!("{}", sum);
}
