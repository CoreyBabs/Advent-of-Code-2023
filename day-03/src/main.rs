use std::cmp;

fn main() {
    let input = include_str!("../input.txt");
    let mut input = input.split("\n").collect::<Vec<&str>>();
    input.pop();
    part1(&input);
}

fn is_symbol(c: char) -> bool {
   !c.is_digit(10) && c != '.'
}

fn substr_has_symbol(substr: &str) -> bool {
    substr.chars().any(|c| is_symbol(c))
}

// this algorithm checks for numbers and then looks for symbols around it
// it might be faster to check for symbols and then look for numbers around it
// because a symbol can have multiple numbers around it
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
