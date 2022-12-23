struct Computer {
    pub cycles: Vec<Cycle>,
    pub register: i64,
    screen: [[char; 40]; 6],
}

impl Computer {
    fn init() -> Self {
        let screen = [['.'; 40]; 6];
        Computer { cycles: Vec::new(), register: 1, screen } 
    }

    fn push(&mut self, instruction: Instruction) {
        let num = self.cycles.len() + 1;
        let cycle = match instruction {
            Instruction::Noop => Cycle::new(num, self.register, instruction),
            Instruction::Addx(val) => { 
                let cycle = Cycle::new(num, self.register, Instruction::Addx(0));
                self.cycles.push(cycle);

                let cycle = Cycle::new(num + 1, self.register, instruction);
                self.register += val;
                cycle
            },
        };
        self.cycles.push(cycle);
    }

    fn draw(&mut self) {
        for cycle in &self.cycles {
            let y: usize = (cycle.num - 1) / 40;
            let x: usize = (cycle.num - 1) % 40; 
            let sprite_position = cycle.during;

            if x as i64 == sprite_position 
                || x as i64 == sprite_position - 1 
                || x as i64 == sprite_position + 1 {
                self.screen[y][x] = '#';
            }
            print!("{}", self.screen[y][x]);

            if x == 39 {
                print!("\r\n");
            }
        }
    }
}

enum Instruction {
    Noop,
    Addx(i64),
}

struct Cycle {
    pub num: usize,
    pub during: i64,
    after: i64,
    instruction: Instruction,
}

impl Cycle {
    fn new(num: usize, x: i64, instruction: Instruction) -> Cycle {
        match instruction {
            Instruction::Noop => Cycle { num, during: x, after: x, instruction },
            Instruction::Addx(val) => Cycle { num, during: x, after: x + val, instruction },
        }
    }

    fn signal(&self) -> i64 {
        self.num as i64 * self.during
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut computer = Computer::init();

    for line in input.lines() {
        let line = line.trim();
        let instruction: Vec<&str> = line.split_whitespace().collect();

        match instruction.len() {
            1 => computer.push(Instruction::Noop),
            2 => {
                let val = instruction[1].parse::<i64>().unwrap();
                computer.push(Instruction::Addx(val))
            },
            _ => unreachable!(),
        }
    }

    let important_cycles = [20, 60, 100, 140, 180, 220];
    let signal_sum = important_cycles.iter()
        .map(|n| computer.cycles.get(n - 1).unwrap())
        .fold(0, |acc, cycle| acc + cycle.signal());

    println!("sum: {}", signal_sum);
    computer.draw();
}
