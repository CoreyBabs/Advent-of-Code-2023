use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let input: Vec<&str> = input.split("\n\n").collect();
    part1(&input);
    part2(&input);
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

fn part2(input: &Vec<&str>) {
    let directions: Vec<char> = input[0].chars().collect();
    let map = construct_map(input[1]);

    let keys: Vec<String> = map.clone().into_keys().filter(|k| k.ends_with("A")).collect();

    println!("{:?}", keys);
    // println!("{:?}", map);
    
    let counts: Vec<usize> = keys.iter().map(|k| {
        let mut key = k;
        let mut found = false;
        let mut count: usize = 0;
        while !found {
            for dir in directions.iter() {
                count += 1;
                if dir == &'R' {
                    key = &map.get(key).unwrap().1;
                }
                else if dir == &'L' {
                    key = &map.get(key).unwrap().0;
                }

                if key.ends_with("Z") {
                    found = true;
                    break;
                }
            }
        }

        count
    })
    .collect();

    let mut lcm = find_lcm(counts[0], counts[1]);

    for &count in &counts[2..] {
        lcm = find_lcm(lcm, count);
    }

    println!("{:?}", lcm);
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

fn gcd(mut a: usize, mut b: usize) -> usize {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

fn find_lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
 }
