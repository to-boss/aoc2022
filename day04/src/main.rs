use std::fmt::Display;

use reader::read_file;

fn main() {
    let input = read_file("input.txt").unwrap();

    let mut counter1 = 0;
    let mut counter2 = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(",").collect();
        let range1 = Range::from_line(parts[0]);
        let range2 = Range::from_line(parts[1]);

        if range1.fully_contains(&range2) || range2.fully_contains(&range1) {
            counter1 += 1;
        }

        if range1.overlap(&range2) {
            counter2 += 1;
        }
    }
    println!("Part 1: {}", counter1);
    println!("Part 2: {}", counter2);
}

struct Range {
    lower: i32,
    upper: i32,
}

impl Range {
    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split("-").collect();
        let lower = parts[0].parse::<i32>().unwrap();
        let upper = parts[1].parse::<i32>().unwrap();
        Self { lower, upper }
    }

    fn range(&self) -> std::ops::Range<i32> {
        self.lower..(self.upper + 1)
    }

    fn overlap(&self, other: &Range) -> bool {
        self.range().any(|n| other.range().contains(&n))
    }

    fn fully_contains(&self, other: &Range) -> bool {
        self.lower >= other.lower && self.upper <= other.upper
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.lower, self.upper)
    }
}
