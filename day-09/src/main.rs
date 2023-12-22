fn main() {
    let input = include_str!("../input.txt");
    let mut input: Vec<&str> = input.split("\n").collect();
    input.pop();

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<&str>) {
    let input: Vec<Vec<&str>> = input.iter()
        .map(|l| l.split(" ").collect::<Vec<&str>>()).collect();

    let input: Vec<Vec<i64>> = input.iter()
        .map(|l| l.iter()
            .map(|i| i.parse::<i64>()
                .expect("Not an i64"))
            .collect())
        .collect();
    
    let mut total = 0;
    for i in input {
        let diffs = get_diffs(&i);
        let diffs = update_diffs_with_preds(diffs);
        total += diffs[0].last().unwrap();
    }

    println!("{:?}", total);
}

fn part2(input: &Vec<&str>) {
    let input: Vec<Vec<&str>> = input.iter()
        .map(|l| l.split(" ").collect::<Vec<&str>>()).collect();

    let input: Vec<Vec<i64>> = input.iter()
        .map(|l| l.iter()
            .map(|i| i.parse::<i64>()
                .expect("Not an i64"))
            .collect())
        .collect();
    
    let mut total = 0;
    for i in input {
        let diffs = get_diffs(&i);
        let diffs = update_diffs_with_preds_front(diffs);
        total += diffs[0][0];
    }

    println!("{:?}", total);
}

fn get_diffs(input: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut diffs: Vec<Vec<i64>> = vec![];
    diffs.push(input.clone());
    while !diffs.last().unwrap_or(&vec![]).iter().all(|d| d == &0) {
        let current = diffs.last().unwrap();

        let mut diff: Vec<i64> = Vec::with_capacity(input.len() - 1);
        for (i, val) in current.iter().enumerate() {
            if i == current.len() - 1 {
                break;
            }

            diff.push(current[i + 1] - val);
        }

        // println!("{:?}", diff);
        diffs.push(diff)
    }

    // println!("{:?}", diffs);
    diffs
}

fn update_diffs_with_preds(mut diffs: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    diffs.last_mut().unwrap().push(0);
    for i in (0..diffs.len() - 1).rev() {
        let prev = &diffs[i+1].clone();
        let prev = prev.last().unwrap();
        let current = &mut diffs[i];
        let pred = current.last().unwrap();
        current.push(prev + pred);
    }

    diffs
}

fn update_diffs_with_preds_front(mut diffs: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    diffs.last_mut().unwrap().insert(0, 0);
    for i in (0..diffs.len() - 1).rev() {
        let prev = &diffs[i+1].clone();
        let prev = prev[0];
        let current = &mut diffs[i];
        let pred = current[0];
        current.insert(0, pred - prev);
    }

    diffs
}
