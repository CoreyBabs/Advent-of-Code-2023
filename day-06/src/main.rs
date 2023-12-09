fn main() {
    let input = include_str!("../input.txt");
    let mut input: Vec<&str> = input.split("\n").collect();
    input.pop();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<&str>) {
    let times: Vec<&str> = input[0].split(":").collect();
    let dists: Vec<&str> = input[1].split(":").collect();
    let times: Vec<&str> = times[1].split_ascii_whitespace().collect();
    let dists: Vec<&str> = dists[1].split_ascii_whitespace().collect();
    
    let times: Vec<usize> = times.iter().map(|t| t.parse().unwrap()).collect();
    let dists: Vec<usize> = dists.iter().map(|t| t.parse().unwrap()).collect();

    let counts: Vec<usize> = times.iter().zip(dists.iter())
        .map(|(t,d)| get_count(t, d)).collect();

    let prod: usize = counts.iter().product();

    println!("{:?}", prod);
}

fn part2(input: &Vec<&str>) {
    let times: Vec<&str> = input[0].split(":").collect();
    let dists: Vec<&str> = input[1].split(":").collect();
    let times: Vec<&str> = times[1].split_ascii_whitespace().collect();
    let dists: Vec<&str> = dists[1].split_ascii_whitespace().collect();
    
    let time: usize = times.join("").parse().unwrap();
    let dist: usize = dists.join("").parse().unwrap();

    let count = get_count(&time, &dist);

    println!("{:?}", count);
}

fn get_count(time: &usize, dist: &usize) -> usize {
    let mut count = 0;
    for i in 0..time+1 {
        let time_diff = time - i;
        let distance = i * time_diff;
        if distance > *dist {
            count += 1;
        }
    }

    count
}


