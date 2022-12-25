use std::fmt::Display;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut forest = string_to_forest(&input);
    forest.compute_visibility();
    forest.print_visibility();
    println!("visible trees: {}", forest.count_visibility());
    println!("highest scenic score: {}", forest.find_highest_scenic_score());
}

fn string_to_forest(s: &str) -> Vec<Vec<Tree>> {
    let mut forest: Vec<Vec<Tree>> = Vec::new();
    for line in s.lines() {
        let line = line.trim();
        let mut tree_row = Vec::with_capacity(line.len());
        for c in line.chars() {
            let height = c.to_digit(10).unwrap() as usize;
            let visible = false;
            tree_row.push(Tree::new(height, visible));
        }
        forest.push(tree_row);
    }
    forest
}

trait Forest {
    fn print_height(&self) {}
    fn print_visibility(&self) {}
    fn compute_visibility(&mut self) {}
    fn count_visibility(&self) -> usize { 0 }
    fn find_highest_scenic_score(&self) -> i64 { -1 }
}

impl Forest for Vec<Vec<Tree>>  {
    fn find_highest_scenic_score(&self) -> i64 {
        fn scenic_score(forest: &Vec<Vec<Tree>>, x: usize, y: usize) -> i64 {
            let mut left = 0;
            let mut top = 0;
            let mut right = 0;
            let mut bottom = 0;

            let current_height = forest[y][x].height;
            let left_iter = &forest[y][0..x];
            for left_x in left_iter.iter().rev() {
                if left_x.height >= current_height {
                    left += 1;
                    break;
                }
                left += 1;
            }

            let right_iter = &forest[y][x + 1..forest[y].len()];
            for right_x in right_iter.iter() {
                if right_x.height >= current_height {
                    right += 1;
                    break;
                }
                right += 1;
            }

            let top_iter = &forest[0..y];
            for top_y in top_iter.iter().rev() {
                if top_y[x].height >= current_height {
                    top += 1;
                    break;
                }
                top += 1;
            }

            let bot_iter = &forest[y + 1..forest[y].len()];
            for bot_y in bot_iter.iter() {
                if bot_y[x].height >= current_height {
                    bottom += 1;
                    break;
                }
                bottom += 1;
            }

            left * top * right * bottom
        } 

        let mut highest_scenic_score = 0;
        for (y, row) in self.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let current_score = scenic_score(self, x, y);
                if current_score > highest_scenic_score {
                    highest_scenic_score = current_score;
                }
            }
        }
        highest_scenic_score
    }

    fn compute_visibility(&mut self) {
        fn line_of_sight(current: &mut Tree, neighbour_height: usize, biggest: &mut usize) {
            if current.height > *biggest && current.height > neighbour_height {
                current.visible = true;
                *biggest = current.height;
            }
        }
        // left check
        for y in 0..self.len() {
            let mut biggest = 0;
            for x in 0..self[y].len() {
                if x == 0 {
                    self[y][x].visible = true;
                    biggest = self[y][x].height;
                } else {
                    let neighbour_height = self[y][x - 1].height;
                    line_of_sight(&mut self[y][x], neighbour_height, &mut biggest);
                }
            }
        }
        // top check
        for x in 0..self.len() {
            let mut biggest = 0;
            for y in 0..self[x].len() {
                if y == 0 {
                    self[y][x].visible = true;
                    biggest = self[y][x].height;
                } else {
                    let neighbour_height = self[y - 1][x].height;
                    line_of_sight(&mut self[y][x], neighbour_height, &mut biggest);
                }
            }
        }
        // right check
        for y in (0..self.len()).rev() {
            let mut biggest = 0;
            for x in (0..self[y].len()).rev() {
                if x == self[y].len() - 1 {
                    self[y][x].visible = true;
                    biggest = self[y][x].height;
                } else {
                    let neighbour_height = self[y][x + 1].height;
                    line_of_sight(&mut self[y][x], neighbour_height, &mut biggest);
                }
            }
        }
        // bottom check
        for x in (0..self.len()).rev() {
            let mut biggest = 0;
            for y in (0..self[x].len()).rev() {
                if y == self[y].len() - 1 {
                    self[y][x].visible = true;
                    biggest = self[y][x].height;
                } else {
                    let neighbour_height = self[y + 1][x].height;
                    line_of_sight(&mut self[y][x], neighbour_height, &mut biggest);
                }
            }
        }
    }

    fn count_visibility(&self) -> usize {
        let mut visible = 0;
        for row in self {
            for tree in row {
                if tree.visible {
                    visible += 1;
                }
            }
        }
        visible
    }

    fn print_height(&self) {
        for row in self {
            for tree in row {
                print!("{}", tree.height);
            }
            print!("\r\n");
        }
    }

    fn print_visibility(&self) {
        for row in self {
            for tree in row {
                print!("{}", tree);
            }
            print!("\r\n");
        }
    }
}

struct Tree {
    pub visible: bool,
    pub height: usize,
}

impl Tree {
    fn new(height: usize, visible: bool) -> Self {
        Tree {
            visible,
            height,
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.visible {
            write!(f, "T")
        } else {
            write!(f, ".")
        }
    }
}
