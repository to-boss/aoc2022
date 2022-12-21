fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Hello, world!");
}

fn read_file(input: String) {
    let mut root = Dir::new("/", None);
    let mut current_dir = Box::new(&mut root);
    let mut input_iter = input.lines();
    while let Some(line) = input_iter.next() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts[0].starts_with("$") {
            let command = parts[1];
            let val = parts[2];

            match command {
                "cd" => match val {
                    ".." => todo!(),
                    "/" => current_dir = Box::new(&mut root),
                    _ => current_dir.add_dir(Dir::new(val, Some(&current_dir))),
                },
                "ls" => {}
                _ => todo!(),
            }
        }
    }
}

struct Dir<'a> {
    name: &'a str,
    parent: Option<&'a Dir<'a>>,
    dirs: Vec<Dir<'a>>,
    files: Vec<File<'a>>,
}

impl<'a> Dir<'a> {
    fn new(name: &'a str, parent: Option<&'a Dir<'a>>) -> Self {
        Dir {
            name,
            parent,
            dirs: Vec::new(),
            files: Vec::new(),
        }
    }

    fn add_file(&mut self, file: File<'a>) {
        self.files.push(file);
    }

    fn add_dir(&mut self, dir: Dir<'a>) {
        self.dirs.push(dir);
    }
}

struct File<'a> {
    name: &'a str,
    size: u32,
}

impl<'a> File<'a> {
    fn new(name: &'a str, size: u32) -> Self {
        File { name, size }
    }
}

#[test]
fn small_input() {
    let input = "$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k";

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts[0].starts_with("$") {
            let command = parts[1];
            let val = parts[2];

            match command {
                "cd" => match val {
                    ".." => println!(""),
                    "/" => println!(""),
                    _ => unreachable!(),
                },
                "ls" => match val {
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            }
        }
    }
}
