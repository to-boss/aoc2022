fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", find_marker(&input, 4));
    println!("{}", find_marker(&input, 14));
}

fn find_marker(input: &str, window_size: usize) -> i32 {
    let mut start = 0;
    let mut end = window_size;
    while end < input.len() {
        let window = &input[start..end];
        if only_uniques(window) {
            return end as i32;
        }
        start += 1;
        end += 1;
    }
    -1
}

fn only_uniques(window: &str) -> bool {
    let mut bitset: Vec<u8> = vec![0; 26];
    for byte in window.bytes() {
        if bitset[(byte - 97) as usize] == 1 {
            return false;
        } else {
            bitset[(byte - 97) as usize] = 1;
        }
    }
    true
}

#[test]
fn test_input_part1() {
    let inputs = vec![
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjwc",
    ];
    let left = vec![5, 6, 10, 11];
    let mut right = Vec::new();

    inputs
        .iter()
        .for_each(|input| right.push(find_marker(&input, 4)));

    assert_eq!(left[0], right[0]);
    assert_eq!(left[1], right[1]);
    assert_eq!(left[2], right[2]);
    assert_eq!(left[3], right[3]);
}

#[test]
fn test_input_part2() {
    let inputs = vec![
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];
    let left = vec![19, 23, 23, 29, 26];
    let mut right = Vec::new();

    inputs
        .iter()
        .for_each(|input| right.push(find_marker(&input, 14)));

    assert_eq!(left[0], right[0]);
    assert_eq!(left[1], right[1]);
    assert_eq!(left[2], right[2]);
    assert_eq!(left[3], right[3]);
    assert_eq!(left[4], right[4]);
}
