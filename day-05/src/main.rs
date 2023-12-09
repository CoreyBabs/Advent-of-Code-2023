use std::vec;

fn main() {
    let input = include_str!("../input.txt");
    let input = input.split("\n\n").collect::<Vec<&str>>();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<&str>) {
    let seeds = input[0].split(": ").collect::<Vec<&str>>();
    let mut seeds: Vec<Seed> = seeds[1].split(" ")
        .map(|s| {
            let mut seed = Seed::default();
            seed.number = s.parse().unwrap();
            seed
        }).collect();

    
    let compartments = vec![
        Compartment::Soil,
        Compartment::Fertilizer,
        Compartment::Water,
        Compartment::Light, 
        Compartment::Temp, 
        Compartment::Humidity, 
        Compartment::Location];
    
    for (i, lines) in input.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let map = construct_map(lines);
        // println!("{}, {:?}", lines, &compartments[i-1]);
        update_seeds(&mut seeds, &map, &compartments[i - 1]);
    }

    let min = seeds.iter()
        .map(|s| s.location)
        .min().unwrap();

    // println!("{:?}", seeds);
    println!("{min}");
}

fn part2(input: &Vec<&str>) {
    let seeds_pairs = input[0].split(": ").collect::<Vec<&str>>();
    let seeds_pairs: Vec<&str> = seeds_pairs[1].split(" ").collect();

    let mut idx = 0;
    let mut mins: Vec<usize> = vec![];
    while idx < seeds_pairs.len() {
        let mut seeds: Vec<usize> = 
            construct_seed_list_from_pairs(seeds_pairs[idx], seeds_pairs[idx + 1]);
        
        for (i, lines) in input.iter().enumerate() {
            if i == 0 {
                continue;
            }

            let map = construct_map(lines);
            // println!("{}, {:?}", lines, &compartments[i-1]);
           update_seed_values_in_place(&mut seeds, &map);
        }


        let min = seeds.iter()
            .map(|s| s)
            .min().unwrap();

        mins.push(*min);
        idx += 2;
    }

    let min = mins.iter().min().unwrap();

    // println!("{:?}", seeds);
    println!("{min}");
}

fn construct_seed_list_from_pairs(start: &str, length: &str) -> Vec<usize> {
    let start: usize = start.parse().unwrap();
    let length: usize = length.parse().unwrap();
    
    (start..start+length).collect()
}

fn update_seeds(seeds: &mut Vec<Seed>, map: &Vec<Map>, compartment: &Compartment) {
    println!("{:?}", compartment);
    let mut current_map: Option<&Map> = None;
    seeds.iter_mut().for_each(|seed| {
        let src = match compartment {
            Compartment::Soil => &seed.number,
            Compartment::Fertilizer => &seed.soil,
            Compartment::Water => &seed.fertilizer,
            Compartment::Light => &seed.water,
            Compartment::Temp => &seed.light,
            Compartment::Humidity => &seed.temperature,
            Compartment::Location => &seed.humidity,
        };

        let mut dst = *src;

        if current_map.is_none() || 
            current_map.is_some_and(|m| src < &m.src || src > &(&m.src + &m.length - 1)) {
            current_map = map.iter()
                .find(|m| m.src <= *src && m.src + m.length - 1 >= *src);
        }

        match current_map {
            Some(mr) => dst = mr.dst + (src - mr.src),
            None => (), 
        }

        match compartment {
            Compartment::Soil => seed.soil = dst,
            Compartment::Fertilizer => seed.fertilizer = dst,
            Compartment::Water => seed.water = dst,
            Compartment::Light => seed.light = dst,
            Compartment::Temp => seed.temperature = dst,
            Compartment::Humidity => seed.humidity = dst,
            Compartment::Location => seed.location = dst,
        }
    });
}

fn update_seed_values_in_place(seeds: &mut Vec<usize>, map: &Vec<Map>) {
    let mut current_map: Option<&Map> = None;
    for i in 0..seeds.len() {
        let src = seeds[i];
        let mut dst = src;

        if current_map.is_none() || 
            current_map.is_some_and(|m| src < m.src || src > m.src + m.length - 1) {
            current_map = map.iter()
                .find(|m| m.src <= src && m.src + m.length - 1 >= src);
        }

        match current_map {
            Some(mr) => dst = mr.dst + (src - mr.src),
            None => (), 
        }

        seeds[i] = dst;
    }
}

fn construct_map(lines: &str) -> Vec<Map> {
    let lines: Vec<&str> = lines.split("\n").collect();
    let values: Vec<Vec<&str>> = lines.iter()
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .collect();

    let mut map: Vec<Map> = Vec::with_capacity(lines.len());

    for (i, value) in values.iter().enumerate() {
        if i == 0 || value.len() != 3 {
            continue;
        }

        let length = value[2].parse().unwrap();
        let src: usize = value[1].parse().unwrap();
        let dst: usize = value[0].parse().unwrap();
        map.push(Map {length, src, dst});
    }

    return map;
}

#[derive(Debug, Default)]
struct Seed {
    number: usize,
    soil: usize,
    fertilizer: usize,
    water: usize,
    light: usize,
    temperature: usize,
    humidity: usize,
    location: usize,
}

#[derive(Debug)]
struct Map {
    src: usize,
    dst: usize,
    length: usize
}

#[derive(Debug)]
enum Compartment {
    Soil,
    Fertilizer,
    Water,
    Light,
    Temp,
    Humidity,
    Location
}
