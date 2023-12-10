use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<&str> = input.split("\n\n").collect();
    part1(&input);
}

fn part1(input: &Vec<&str>) {
    let directions: Vec<char> = input[0].chars().collect();
    let map = construct_map(input[1]);
    // println!("{:?}", map);

    let mut key = "AAA";
    let mut found = false;
    let mut count = 0;
    while !found {
        for dir in directions.iter() {
            count += 1;
            if dir == &'R' {
                key = &map.get(key).unwrap().1;
            }
            else if dir == &'L' {
                key = &map.get(key).unwrap().0;
            }

            if key == "ZZZ" {
                found = true;
                break;
            }
        }
    }

    
    println!("{:?}", count);
}

fn construct_map(input: &str) -> HashMap<String, (String, String)> {
    let mut map_lines: Vec<&str> = input.split("\n").collect();
    map_lines.pop();

    let mut map = HashMap::new();
    
    map_lines.iter().for_each(|l| {
        let kv: Vec<&str> = l.split("=").collect();
        let k = kv[0].trim().to_string();
        let v = (kv[1][2..5].to_string(), kv[1][7..10].to_string());
        map.insert(k, v);
    });

    map
}
