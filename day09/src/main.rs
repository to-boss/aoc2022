use std::fmt::Display;
use std::collections::HashSet;

const LABELS: [char; 10] = ['H', '1', '2', '3', '4', '5', '6', '7', '8', 'T'];

fn main() -> Result<(), String> {
    let input = std::fs::read_to_string("input.txt")
        .expect("Should be able to read string");

    let directions = parse_directions(&input)?;

    // part 1
    part1(&directions);

    // part 2
    part2(&directions);
    Ok(())
}

fn part2(directions: &Vec<Direction>) {
    let mut rope: Vec<Node> = Vec::with_capacity(10);
    for _ in 0..10 {
        let node = Node { x: 0, y: 0 };
        rope.push(node);
    }
    let mut tail_positions = HashSet::new();
    let last = rope.len() - 1;

    tail_positions.insert((rope[last].x, rope[last].y));
    for dir in directions {
        rope[0].r#move(dir);

        for i in 1..rope.len() {
            while !rope[i].touches(&rope[i - 1]) {
                let directions = rope[i - 1].drag(&rope[i]);
                directions.iter().for_each(|dir| rope[i].r#move(dir));

                if i == last {
                    tail_positions.insert((rope[last].x, rope[last].y));
                }
            }
        }
    }
    println!("{}", tail_positions.len());
}

fn part1(directions: &Vec<Direction>) {
    let mut head = Node { x: 0, y: 0 };
    let mut tail = Node { x: 0, y: 0 };
    let mut tail_positions = HashSet::new();

    tail_positions.insert((tail.x, tail.y));
    for dir in directions {
        head.r#move(dir);

        while !tail.touches(&head) {
            let directions = head.drag(&tail);
            directions.iter().for_each(|dir| tail.r#move(dir));

            tail_positions.insert((tail.x, tail.y));
        }
    }
    println!("{}", tail_positions.len());
}

#[allow(dead_code)]
struct Board {
    pub field: [[char; 6]; 5],
}

#[allow(dead_code)]
impl Board {

    fn print(&mut self, nodes: &Vec<Node>) {
        self.field.iter_mut()
            .flat_map(|row| row.iter_mut())
            .for_each(|c| *c = '.');

        for i in 0..nodes.len() {
            let x = nodes[i].x as usize; 
            let y = nodes[i].y as usize;
            self.field[y + 4][x] = LABELS[i];
        }

        for y in 0..self.field.len() {
            for x in 0..self.field[0].len() {
                print!("{}", self.field[y][x]);
            }
            print!("\r\n");
        }
        print!("\r\n");
    }
}

fn parse_directions(input: &str) -> Result<Vec<Direction>, String> {
    let mut directions = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        let dirs: Vec<Direction> = Direction::from_str(line).unwrap();
        for dir in dirs {
            directions.push(dir);
        }
    }
    Ok(directions)
}

enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn from_str(s: &str) -> Result<Vec<Self>, String> {
        let (direction, count) = s.split_once(' ').unwrap();
        let count: i64 = count.parse().unwrap();
        (0..count).into_iter().map(|_| match direction {
                "R" => Ok(Direction::Right),
                "L" => Ok(Direction::Left),
                "U" => Ok(Direction::Up),
                "D" => Ok(Direction::Down),
                _ => unreachable!(),
        }).collect()
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Right => write!(f, "== RIGHT 1 =="),
            Self::Left => write!(f, "== LEFT 1 =="),
            Self::Up => write!(f, "== UP 1 =="),
            Self::Down => write!(f, "== DOWN 1 =="),
        }
    }
}

struct Node {
    x: i64,
    y: i64,
}

impl Node {
    fn r#move(&mut self, dir: &Direction) {
        match dir {
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
        }
    }

    fn touches(&self, other: &Node) -> bool {
        let x_diff = self.x - other.x;
        let y_diff = self.y - other.y;
        x_diff.abs() <= 1 && y_diff.abs() <= 1
    }

    fn drag(&self, other: &Node) -> Vec<Direction> {
        let mut directions = Vec::new();
        // top-left
        if self.x < other.x && self.y < other.y {
            directions.push(Direction::Up);
            directions.push(Direction::Left);
        }
        // top-right
        else if self.x > other.x && self.y < other.y  {
            directions.push(Direction::Up);
            directions.push(Direction::Right);
        }
        // bot-left
        else if self.x < other.x && self.y > other.y  {
            directions.push(Direction::Down);
            directions.push(Direction::Left);
        }
        // bot-right
        else if self.x > other.x && self.y > other.y  {
            directions.push(Direction::Down);
            directions.push(Direction::Right);
        }
        // right
        else if self.x > other.x {
            directions.push(Direction::Right);
        // left
        } else if self.x < other.x {
            directions.push(Direction::Left);
        // up
        } else if self.y > other.y {
            directions.push(Direction::Down);
        // down
        } else if self.y < other.y {
            directions.push(Direction::Up);
        } else {
            unreachable!();
        }
        directions
    }
}
