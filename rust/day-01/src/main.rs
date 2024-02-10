use std::usize::MAX;

fn main() {
    let input = include_str!("../input.txt");
    let input = input.split("\n").collect::<Vec<&str>>();
    part1(&input);
    part2(&input);
    // println!("{}", input[0]);
}

fn first_and_last_to_number(line: &Vec<char>) -> u32 {
    if line.len() == 0 {
        return 0;
    }

    let mut amount = "".to_string();
    amount.push(*line.first().unwrap());
    amount.push(*line.last().unwrap());
    amount.parse::<u32>().unwrap()
}

fn replace_words_with_digits(line: &str, converter: &Vec<Digit>) -> u32 {
    if line.len() == 0 {
        return 0;
    }

    let contains_word: Vec<(char, Vec<_>)> = converter.iter()
        .filter(|c| line.contains(c.word))
        .map(|c| (c.digit, line.match_indices(c.word).map(|m| m.0).collect())).collect();

    let contains_digit: Vec<(char, Vec<_>)> = converter.iter()
        .filter(|c| line.contains(c.digit))
        .map(|c| (c.digit, line.match_indices(c.digit).map(|m| m.0).collect())).collect();

    // println!("{}", line);
    // println!("{:?}", contains_word);
    // println!("{:?}", contains_digit);

    let mut min = (MAX, '0');
    let mut max = (0, '0');
    for digit in contains_word {
        for idx in digit.1 {
            if idx >= max.0 {
               max = (idx, digit.0); 
            }
            if idx < min.0 {
               min = (idx, digit.0); 
            }
        }
    }

    for digit in contains_digit {
        for idx in digit.1 {
            if idx >= max.0 {
               max = (idx, digit.0); 
            }
            if idx < min.0 {
               min = (idx, digit.0); 
            }
        }
    }
    
    // println!("{:?}", min);
    // println!("{:?}", max);
    first_and_last_to_number(&vec![min.1, max.1])
}

fn part1(input: &Vec<&str>) {
    let digits: Vec<Vec<char>> = input.iter()
    .map(|l| l.chars().filter(|c| c.is_digit(10)).collect())
    .collect();

    let sum: u32 = digits.iter()
        .map(|d| first_and_last_to_number(&d))
        .sum();

    println!("{}", sum);
}

fn part2(input: &Vec<&str>) {
    let converter = vec![
        Digit::new("one", '1'),
        Digit::new("two", '2'),
        Digit::new("three", '3'),
        Digit::new("four", '4'),
        Digit::new("five", '5'),
        Digit::new("six", '6'),
        Digit::new("seven", '7'),
        Digit::new("eight", '8'),
        Digit::new("nine", '9')];

    let sum: u32 = input.iter()
        .map(|d| replace_words_with_digits(&d, &converter))
        .sum();

    println!("{}", sum);
}

struct Digit<'a> {
    word: &'a str,
    digit: char,
}

impl Digit<'_> {
    fn new<'a>(word: &'a str, digit: char) -> Digit<'a> {
        Digit { word, digit }
    }
}


#[test]
fn overlapping() {
    let input = "eightwothree";
    let converter = vec![
        Digit::new("one", '1'),
        Digit::new("two", '2'),
        Digit::new("three", '3'),
        Digit::new("four", '4'),
        Digit::new("five", '5'),
        Digit::new("six", '6'),
        Digit::new("seven", '7'),
        Digit::new("eight", '8'),
        Digit::new("nine", '9')];
    let result = replace_words_with_digits(input, &converter);
    assert_eq!(result, 83);
}

#[test]
fn words_and_numbers() {
    let input = "two1nine";
    let converter = vec![
        Digit::new("one", '1'),
        Digit::new("two", '2'),
        Digit::new("three", '3'),
        Digit::new("four", '4'),
        Digit::new("five", '5'),
        Digit::new("six", '6'),
        Digit::new("seven", '7'),
        Digit::new("eight", '8'),
        Digit::new("nine", '9')];
    let result = replace_words_with_digits(input, &converter);
    assert_eq!(result, 29);
}


#[test]
fn words_and_numbers2() {
    let input = "abcone2threexyz";
    let converter = vec![
        Digit::new("one", '1'),
        Digit::new("two", '2'),
        Digit::new("three", '3'),
        Digit::new("four", '4'),
        Digit::new("five", '5'),
        Digit::new("six", '6'),
        Digit::new("seven", '7'),
        Digit::new("eight", '8'),
        Digit::new("nine", '9')];
    let result = replace_words_with_digits(input, &converter);
    assert_eq!(result, 13);
}

#[test]
fn words_and_numbers3() {
    let input = "xtwone3four";
    let converter = vec![
        Digit::new("one", '1'),
        Digit::new("two", '2'),
        Digit::new("three", '3'),
        Digit::new("four", '4'),
        Digit::new("five", '5'),
        Digit::new("six", '6'),
        Digit::new("seven", '7'),
        Digit::new("eight", '8'),
        Digit::new("nine", '9')];
    let result = replace_words_with_digits(input, &converter);
    assert_eq!(result, 24);
}

#[test]
fn words_and_numbers4() {
    let input = "4nineeightseven2";
    let converter = vec![
        Digit::new("one", '1'),
        Digit::new("two", '2'),
        Digit::new("three", '3'),
        Digit::new("four", '4'),
        Digit::new("five", '5'),
        Digit::new("six", '6'),
        Digit::new("seven", '7'),
        Digit::new("eight", '8'),
        Digit::new("nine", '9')];
    let result = replace_words_with_digits(input, &converter);
    assert_eq!(result, 42);
}

#[test]
fn words_and_numbers5() {
    let input = "zoneight234";
    let converter = vec![
        Digit::new("one", '1'),
        Digit::new("two", '2'),
        Digit::new("three", '3'),
        Digit::new("four", '4'),
        Digit::new("five", '5'),
        Digit::new("six", '6'),
        Digit::new("seven", '7'),
        Digit::new("eight", '8'),
        Digit::new("nine", '9')];
    let result = replace_words_with_digits(input, &converter);
    assert_eq!(result, 14);
}

#[test]
fn with_teens() {
    let input = "7pqrstsixteen";
    let converter = vec![
        Digit::new("one", '1'),
        Digit::new("two", '2'),
        Digit::new("three", '3'),
        Digit::new("four", '4'),
        Digit::new("five", '5'),
        Digit::new("six", '6'),
        Digit::new("seven", '7'),
        Digit::new("eight", '8'),
        Digit::new("nine", '9')];
    let result = replace_words_with_digits(input, &converter);
    assert_eq!(result, 76);
}

#[test]
fn single_digit() {
    let input = "treb7uchet";
    let converter = vec![
        Digit::new("one", '1'),
        Digit::new("two", '2'),
        Digit::new("three", '3'),
        Digit::new("four", '4'),
        Digit::new("five", '5'),
        Digit::new("six", '6'),
        Digit::new("seven", '7'),
        Digit::new("eight", '8'),
        Digit::new("nine", '9')];
    let result = replace_words_with_digits(input, &converter);
    assert_eq!(result, 77);
}


#[test]
fn single_word() {
    let input = "trebsevenuchet";
    let converter = vec![
        Digit::new("one", '1'),
        Digit::new("two", '2'),
        Digit::new("three", '3'),
        Digit::new("four", '4'),
        Digit::new("five", '5'),
        Digit::new("six", '6'),
        Digit::new("seven", '7'),
        Digit::new("eight", '8'),
        Digit::new("nine", '9')];
    let result = replace_words_with_digits(input, &converter);
    assert_eq!(result, 77);
}

#[test]
fn duplicate_digits_and_words() {
    let input = "zoneight21384";
    let converter = vec![
        Digit::new("one", '1'),
        Digit::new("two", '2'),
        Digit::new("three", '3'),
        Digit::new("four", '4'),
        Digit::new("five", '5'),
        Digit::new("six", '6'),
        Digit::new("seven", '7'),
        Digit::new("eight", '8'),
        Digit::new("nine", '9')];
    let result = replace_words_with_digits(input, &converter);
    assert_eq!(result, 14);
}

