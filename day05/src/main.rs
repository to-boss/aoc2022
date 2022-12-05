use std::{collections::VecDeque, fmt::Display};

use reader::read_file;

fn main() {
    let input = read_file("input.txt").unwrap();

    let mut state: Vec<&str> = input.lines().take_while(|line| !line.is_empty()).collect();

    let amount_of_states = state
        .pop()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks: Vec<VecDeque<char>> = Vec::with_capacity(amount_of_states);
    for _ in 0..amount_of_states {
        stacks.push(VecDeque::new());
    }

    // read state and put in stacks
    for line in &state {
        let mut step = 0;
        let mut i = 0;
        let chars: Vec<char> = line.chars().skip(1).collect();
        while let Some(char) = chars.get(step) {
            if !char.is_whitespace() {
                stacks[i].push_back(*char);
            }
            step += 4;
            i += 1;
        }
    }

    // read instructions
    let instructions: Vec<Instruction> = input
        .lines()
        .skip(state.len() + 2)
        .map(|line| Instruction::from_line(line))
        .collect();

    // println!("{:?}", stacks);
    for instruction in instructions {
        //instruction.execute(&mut stacks);
        instruction.execute_multi(&mut stacks);
        // println!("{:?}", stacks);
    }

    for stack in stacks.iter_mut() {
        print!("{}", stack.pop_front().unwrap())
    }
}

struct Instruction {
    take: usize,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from_line(line: &str) -> Self {
        let nums: Vec<usize> = line
            .split_whitespace()
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, c)| c.parse::<usize>().unwrap())
            .collect();
        let take = nums[0];
        let from = nums[1];
        let to = nums[2];

        Instruction { take, from, to }
    }

    fn execute(&self, state: &mut Vec<VecDeque<char>>) {
        for _ in 0..self.take {
            let moving = state[self.from - 1].pop_front().unwrap();
            state[self.to - 1].push_front(moving);
        }
    }

    fn execute_multi(&self, state: &mut Vec<VecDeque<char>>) {
        if self.take == 1 {
            self.execute(state);
        } else {
            let moving: Vec<char> = state[self.from - 1].drain(..self.take).collect();
            for element in moving.iter().rev() {
                state[self.to - 1].push_front(*element);
            }
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "take {} from {} to {}", self.take, self.from, self.to)
    }
}
