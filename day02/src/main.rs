use reader::read_file;

fn main() {
    let input = read_file("input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|l| l.to_owned()).collect();

    // Part 1
    let mut total_score = 0;
    for line in lines.iter() {
        total_score += calc_score_a(&line);
    }
    println!("Part 1: {}", total_score);

    // Part 2
    let mut total_score = 0;
    for line in lines.iter() {
        total_score += calc_score_b(&line);
    }
    println!("Part 2: {}", total_score);
}

fn calc_score_b(line: &String) -> i32 {
    let handshapes: Vec<&str> = line.split_whitespace().collect();
    let opponent = &HandShape::from_str(handshapes[0]);
    let ending = &Ending::from_str(handshapes[1]);
    opponent.play_rigged(ending)
}

fn calc_score_a(line: &String) -> i32 {
    let handshapes: Vec<HandShape> = line
        .split_whitespace()
        .map(|s| HandShape::from_str(s))
        .collect();
    let opponent = &handshapes[0];
    let me = &handshapes[1];
    me.play(opponent)
}

enum Ending {
    Loose = 0,
    Draw = 3,
    Win = 6,
}

impl Ending {
    fn from_str(s: &str) -> Self {
        assert!(s.len() == 1);
        match s {
            "X" => Ending::Loose,
            "Y" => Ending::Draw,
            "Z" => Ending::Win,
            _ => panic!("Invalid char"),
        }
    }
}

enum HandShape {
    Rock,
    Paper,
    Scissors,
}

impl HandShape {
    fn from_str(s: &str) -> Self {
        assert!(s.len() == 1);
        match s {
            "A" | "X" => HandShape::Rock,
            "B" | "Y" => HandShape::Paper,
            "C" | "Z" => HandShape::Scissors,
            _ => panic!("Invalid char"),
        }
    }

    fn score(&self) -> i32 {
        match self {
            HandShape::Rock => 1,
            HandShape::Paper => 2,
            HandShape::Scissors => 3,
        }
    }

    fn play(&self, other: &HandShape) -> i32 {
        let score = match self {
            HandShape::Rock => match other {
                HandShape::Rock => Ending::Draw as i32,
                HandShape::Paper => Ending::Loose as i32,
                HandShape::Scissors => Ending::Win as i32,
            },
            HandShape::Paper => match other {
                HandShape::Rock => Ending::Win as i32,
                HandShape::Paper => Ending::Draw as i32,
                HandShape::Scissors => Ending::Loose as i32,
            },
            HandShape::Scissors => match other {
                HandShape::Rock => Ending::Loose as i32,
                HandShape::Paper => Ending::Win as i32,
                HandShape::Scissors => Ending::Draw as i32,
            },
        };
        score + self.score()
    }

    fn play_rigged(&self, other: &Ending) -> i32 {
        let chosen: HandShape = match self {
            HandShape::Rock => match other {
                Ending::Loose => HandShape::Scissors,
                Ending::Draw => HandShape::Rock,
                Ending::Win => HandShape::Paper,
            },
            HandShape::Paper => match other {
                Ending::Loose => HandShape::Rock,
                Ending::Draw => HandShape::Paper,
                Ending::Win => HandShape::Scissors,
            },
            HandShape::Scissors => match other {
                Ending::Loose => HandShape::Paper,
                Ending::Draw => HandShape::Scissors,
                Ending::Win => HandShape::Rock,
            },
        };
        chosen.play(self) + chosen.score()
    }
}
