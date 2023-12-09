fn main() {
    let input = include_str!("../input.txt");
    let mut input: Vec<&str> = input.split("\n").collect();
    input.pop();
    part1(&input);
}

fn part1(input: &Vec<&str>) {
    let times: Vec<&str> = input[0].split(":").collect();
    let dists: Vec<&str> = input[1].split(":").collect();
    let times: Vec<&str> = times[1].split_ascii_whitespace().collect();
    let dists: Vec<&str> = dists[1].split_ascii_whitespace().collect();
    
    let times: Vec<u32> = times.iter().map(|t| t.parse().unwrap()).collect();
    let dists: Vec<u32> = dists.iter().map(|t| t.parse().unwrap()).collect();

    let counts: Vec<u32> = times.iter().zip(dists.iter())
        .map(|(t,d)| get_count(t, d)).collect();

    let prod: u32 = counts.iter().product();

    println!("{:?}", prod);
}

fn get_count(time: &u32, dist: &u32) -> u32 {
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


