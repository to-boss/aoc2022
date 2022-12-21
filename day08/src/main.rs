use std::fmt::{write, Display};

fn main() {
    println!("Hello, world!");
}

fn top_left(trees: &mut Vec<Tree>, input: &str) {
    for (y, line) in input.lines().enumerate() {
        let line = line.trim().as_bytes().iter().enumerate();
        for (x, height) in line {
            let height = (height - 48) as usize;
            let mut visible = false;
            if is_at_border(x, y) {
                visible = true;
            } else {
            }

            let tree = Tree::new(x, y, height, visible);
            trees.push(tree);
        }
        print!("\r\n");
    }
}

fn is_at_border(x: usize, y: usize) -> bool {
    if x == 0 || y == 0 {
        return true;
    }
    false
}

struct Tree {
    visible: bool,
    height: usize,
    x: usize,
    y: usize,
}

impl Tree {
    fn new(x: usize, y: usize, height: usize, visible: bool) -> Self {
        Tree {
            visible,
            height,
            x,
            y,
        }
    }

    fn forest(trees: Vec<Tree>, line_len: usize) {
        for (i, tree) in trees.iter().enumerate() {
            let i = i + 1;
            print!("{}", tree);
            if i % line_len == 0 {
                print!("\r\n");
            }
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.visible {
            write!(f, "T")
        } else {
            write!(f, "#")
        }
    }
}

#[test]
fn small_input() {
    let input = "30373
                25512
                65332
                33549
                35390";
    let mut left = 16;
    let mut trees: Vec<Tree> = Vec::new();
    top_left(&mut trees, input);
    Tree::forest(trees, 5);

    // assert_eq!(left, 16);
    // left = 21;
    assert_eq!(left, 21);
}
