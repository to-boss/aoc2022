use reader::read_file;

fn main() {
    let input = read_file("input.txt").unwrap();

    // Part 1
    let mut calories = count_calories(&input);
    let max_calories = calories.iter().max().unwrap().clone();
    println!("Answer A: {:?}", max_calories);

    // Part 2
    calories.sort_by(|a, b| b.cmp(a));
    let top3_sum: i32 = calories.iter().take(3).sum();
    println!("Answer B: {:?}", top3_sum);
}

fn count_calories(input: &String) -> Vec<i32> {
    let mut calories = Vec::new();
    let mut current_elve = 0;
    for line in input.lines() {
        if line.is_empty() {
            calories.push(current_elve);
            current_elve = 0;
        } else {
            current_elve += line.parse::<i32>().unwrap();
        }
    }
    calories
}
