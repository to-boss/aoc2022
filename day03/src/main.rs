use std::collections::HashMap;

use reader::read_file;

fn main() {
    let input = read_file("input.txt").unwrap();

    let start = std::time::Instant::now();

    let lines: Vec<&str> = input.lines().collect();
    let mut total_sum = 0;

    // Part 1
    for line in &lines {
        let (part1, part2) = line.split_at(line.len() / 2);
        let mut letters_p1: HashMap<char, i32> = HashMap::new();
        let mut letters_p2: HashMap<char, i32> = HashMap::new();
        for c in part1.chars() {
            letters_p1
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for c in part2.chars() {
            letters_p2
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        let mut doubles: Vec<char> = Vec::new();
        for key in letters_p1.keys() {
            if letters_p2.contains_key(key) {
                doubles.push(*key);
            }
        }
        let sum: i32 = doubles.iter().fold(0, |acc, c| acc + priority(*c) as i32);
        total_sum += sum;
    }
    println!("{}", total_sum);

    // Part 2
    // Iterate over 3 lines
    total_sum = 0;
    let mut lines_iter = lines.iter();
    while let (Some(l1), Some(l2), Some(l3)) =
        (lines_iter.next(), lines_iter.next(), lines_iter.next())
    {
        let mut elve1: HashMap<char, i32> = HashMap::new();
        let mut elve2: HashMap<char, i32> = HashMap::new();
        let mut elve3: HashMap<char, i32> = HashMap::new();
        for c in l1.chars() {
            elve1
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for c in l2.chars() {
            elve2
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        for c in l3.chars() {
            elve3
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
        let mut doubles: Vec<char> = Vec::new();
        let mut triples: Vec<char> = Vec::new();
        for key in elve1.keys() {
            if elve2.contains_key(key) {
                doubles.push(*key);
            }
        }
        for key in doubles {
            if elve3.contains_key(&key) {
                triples.push(key);
            }
        }
        let sum: i32 = triples.iter().fold(0, |acc, c| acc + priority(*c) as i32);
        total_sum += sum;
    }
    println!("{}", total_sum);
    let dur = start.elapsed().as_micros();
    println!("{dur}");
}

fn priority(c: char) -> u8 {
    match c {
        'a'..='z' => c as u8 - 96,
        'A'..='Z' => c as u8 - 38,
        _ => panic!("Can't match char"),
    }
}
