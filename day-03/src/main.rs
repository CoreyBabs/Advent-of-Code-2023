use std::cmp;

fn main() {
    let input = include_str!("../input.txt");
    let mut input = input.split("\n").collect::<Vec<&str>>();
    input.pop();
    part1(&input);
    part2(&input);
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Previous,
    Next,
    BothPrevious,
    BothNext,
    LeftAndRight,
    None
}

fn is_symbol(c: char) -> bool {
   !c.is_digit(10) && c != '.'
}

fn substr_has_symbol(substr: &str) -> bool {
    substr.chars().any(|c| is_symbol(c))
}

fn get_number_from_substr(substr: &str, direction: Direction) -> usize {
    match direction {
        Direction::Right => substr.chars()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .expect(&format!("{}, {:?}", substr, direction)), 
        Direction::Left => substr.chars()
            .rev()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .chars().rev()
            .collect::<String>()
            .parse()
            .expect(&format!("{}, {:?}", substr, direction)), 
        _ => panic!("Invalid direction")
    }
}

fn get_number_from_line(line: &str, c: usize) -> usize {
    let before = &line[0..c+1];
    let after = &line[c+1..line.len()];

    let mut before = get_number_from_substr(before, Direction::Left).to_string();
    let after = get_number_from_substr(after, Direction::Right).to_string();

    before.push_str(&after);
    before.parse().unwrap()
}

fn count_numbers_in_substr(substr: &str) -> u32 {
    if substr.len() != 3 {
        panic!("Invalid substr length");
    }

    let chars: Vec<char> = substr.chars().collect();

    // check if there are any digits
    if !chars.iter().any(|c| c.is_digit(10)) {
        return 0;
    }

    // All 3 characters are digits, so there is 1 number in the string
    if substr.parse::<u32>().is_ok() {
        return 1;
    }

    // There are digits in the first and last spots, we can infer there is
    // nothing in the center due to the previous check, so there are 2 digits
    if chars[0].is_digit(10) && chars[2].is_digit(10) {
        return 2;
    }

    // We know that there are not 2 or 0 digits, so there must be at least
    // one. This function is not concerned with the position so we can just return 1 here
    1
}

fn get_line_substr_and_direction(line: &str, c: usize) -> (&str, Direction) {
    let chars: Vec<char> = line.chars().collect();
    let before_char_idx = cmp::max(0, c as i32 - 1) as usize;
    let after_char_idx = cmp::min(line.len() - 1, c + 1);
    if !chars[c].is_digit(10) && chars[before_char_idx].is_digit(10) {
        return (&line[0..c], Direction::Left);
    }

    if !chars[c].is_digit(10) && chars[after_char_idx].is_digit(10) {
        return (&line[after_char_idx..line.len()], Direction::Right);
    }

    // Need to check left and right so return whole string going right
    if chars[after_char_idx].is_digit(10) && chars[before_char_idx].is_digit(10) {
        return (line, Direction::LeftAndRight)
    }

    if chars[after_char_idx].is_digit(10) {
        return (&line[c..line.len()], Direction::Right)
    }

    if chars[before_char_idx].is_digit(10) {
        return (&line[0..after_char_idx], Direction::Left)
    }
    
    if !chars[before_char_idx].is_digit(10) && 
      !chars[after_char_idx].is_digit(10) {
        return (&line[c..c+1], Direction::Right)
    }

    panic!("No digits in the string.");
}

// this algorithm checks for numbers and then looks for symbols around it
// it might be faster to check for symbols and then look for numbers around it
// because a symbol can have multiple numbers around it
// After reading part 2, it would have been better to go by symbols
fn part1(input: &Vec<&str>) {
    let mut sum = 0;
    for i in 0..input.len() {
        let line = input[i];

        let prev_line = i as i32 - 1;
        let next_line = i + 1;
        
        let chars: Vec<char> = line.chars().collect();
        let mut c = 0;
        while c < line.len() {
            if !chars[c].is_digit(10) {
                c += 1;
                continue; 
            }

            let mut last_idx = c + 1;
            for n in c + 1..line.len() {
                if !chars[n].is_digit(10) {
                    break;
                }

                last_idx = n + 1;
            }

            let num = &line[c..last_idx];
            let num: u32 = num.parse().expect(&format!("{}, {}, {}\n{}", c, num, i, line).to_string()); 

            let before_char_idx = cmp::max(0, c as i32 - 1) as usize;
            let after_char_idx = cmp::min(line.len() - 1, last_idx);

            c = last_idx + 1; 

            if is_symbol(chars[before_char_idx]) {
                sum += num;
                // println!("{}", num);
                continue;
            }

            if is_symbol(chars[after_char_idx]) {
                // println!("{}", num);
                sum += num;
                continue;
            }

            if prev_line >= 0 &&
              substr_has_symbol(&input[prev_line as usize][before_char_idx..after_char_idx + 1]) {
                // println!("{}", num);
                sum += num;
                continue;
            }

            if next_line < input.len() &&
              substr_has_symbol(&input[next_line][before_char_idx..after_char_idx + 1]) {
                // println!("{}", num);
                sum += num;
                continue;
            }
        }
    }

    println!("{}", sum);
}

fn part2(input: &Vec<&str>) {
    let mut sum: usize = 0;
    for i in 0..input.len() {
        let line = input[i];

        let prev_line = i as i32 - 1;
        let next_line = i + 1;
        
        let chars: Vec<char> = line.chars().collect();
        let mut c = 0;
        while c < line.len() {
            if chars[c] != '*' {
                c += 1;
                continue; 
            }

            let before_char_idx = cmp::max(0, c as i32 - 1) as usize;
            let after_char_idx = cmp::min(line.len() - 1, c + 1);
            let mut num_count = 0;

            // tracks if the digit is left, right, previous, next
            let mut locations = vec![Direction::None, Direction::None, Direction::None, Direction::None];

            // check left
            if chars[before_char_idx].is_digit(10) {
                locations[0] = Direction::Left;
                num_count += 1;
            }

            // check right
            if chars[after_char_idx].is_digit(10) {
                locations[1] = Direction::Right;
                num_count += 1;
            }

            // check previous line
            if prev_line >= 0 { 
                let amount = count_numbers_in_substr(&input[prev_line as usize][before_char_idx..after_char_idx + 1]);
                num_count += amount;
                match amount {
                    1 => locations[2] = Direction::Previous,
                    2 => locations[2] = Direction::BothPrevious,
                    _ => ()
                }
            }

            // check next line
            if next_line < input.len() {
                let amount = count_numbers_in_substr(&input[next_line][before_char_idx..after_char_idx + 1]);
                num_count += amount;
                match amount {
                    1 => locations[3] = Direction::Next,
                    2 => locations[3] = Direction::BothNext,
                    _ => ()
                }
            }

            if num_count != 2 {
                c += 1;
                continue;
            }

            let mut numbers: Vec<usize> = vec![];
            for location in locations {
                match location {
                    Direction::Left => numbers.push(get_number_from_substr(&line[0..before_char_idx+1], location)),
                    Direction::Right => numbers.push(get_number_from_substr(&line[after_char_idx..line.len()], location)),
                    Direction::Previous => {
                        let (substr, direction) = get_line_substr_and_direction(input[prev_line as usize], c);
                        match direction {
                            Direction::Left |
                            Direction::Right => numbers.push(get_number_from_substr(substr, direction)),
                            Direction::LeftAndRight => numbers.push(get_number_from_line(substr, c)),
                            _ => ()
                        }
                    },
                    Direction::Next => {
                        let (substr, direction) = get_line_substr_and_direction(input[next_line], c);
                        match direction {
                            Direction::Left |
                            Direction::Right => numbers.push(get_number_from_substr(substr, direction)),
                            Direction::LeftAndRight => numbers.push(get_number_from_line(substr, c)),
                            _ => ()
                        }
                    },
                    Direction::BothPrevious => {
                        numbers.push(get_number_from_substr(&input[prev_line as usize][0..before_char_idx+1], Direction::Left));
                        numbers.push(get_number_from_substr(&input[prev_line as usize][after_char_idx..line.len()], Direction::Right));
                    },
                    Direction::BothNext => {
                        numbers.push(get_number_from_substr(&input[next_line as usize][0..before_char_idx+1], Direction::Left));
                        numbers.push(get_number_from_substr(&input[next_line][after_char_idx..line.len()], Direction::Right));
                    },
                    _ => ()
                }
            }

            if numbers.len() != 2 {
                panic!("Incorrect amount of numbers to multiply");
            }

            println!("{:?}, {}", numbers, i);

            sum += numbers[0] * numbers[1];
            c += 1;
        }
    }

    println!("{}", sum);
}

#[test]
fn single_digit_line() {
    let line = "...4...";
    
    let mut num: &str = "";
    let mut c = 0;
    let chars: Vec<char> = line.chars().collect();
    while c < line.len() {
        if !chars[c].is_digit(10) {
            c += 1;
            continue; 
        }

        let mut last_idx = c + 1;
        for n in c + 1..line.len() {
            if !chars[n].is_digit(10) {
                break;
            }

            last_idx = n + 1;
        }

        println!("{}, {}", c, last_idx);
        num = &line[c..last_idx];
        c += 1;
    }

    assert_eq!("4", num);
}

#[test]
fn test_single_digit() {
    let result = get_number_from_substr("3", Direction::Right);
    assert_eq!(3, result);
}

#[test]
fn test_three_digits() {
    let test = "..123..";
    let result = get_number_from_line(test, 3);
    assert_eq!(123, result);
}
