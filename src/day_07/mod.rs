use crate::read::read_file;

pub fn run() {
    if let Ok(contents) = read_file("src/day_07/input.txt") {
        println!("part 1: {}", part1(&contents));
        println!("------------------------------------------");
        println!("part 2: {}", part2(&contents));
        // println!("part 2: {}", part2(&contents));
    }
}

#[derive(Debug)]
enum CommandLine<'a> {
    ChangeDirectory(&'a str),
    List,
    File { _name: &'a str, size: u32 },
    Directory { name: &'a str },
}

#[derive(Debug, Default)]
struct ArenaTree {
    arena: Vec<TreeNode>,
}

impl ArenaTree {
    fn parse(contents: &str) -> Self {
        let mut arena_tree = Self::default();
        let mut history = vec![""];
        let mut path = history.join("/");
        if path.is_empty() {
            path = "/".to_string();
        }
        let root_idx = arena_tree.node(&path);
        let mut current_idx = root_idx;
        for line in contents.lines() {
            let command = parse_line(line);
            match command {
                CommandLine::ChangeDirectory(directory) => match directory {
                    ".." => {
                        history.pop();
                        path = history.join("/");
                        if path.is_empty() {
                            path = "/".to_string();
                        }
                        current_idx = arena_tree.node(&path);
                    }
                    "/" => {
                        history = vec![""];
                        path = history.join("/");
                        if path.is_empty() {
                            path = "/".to_string();
                        }
                    }
                    dir => {
                        history.push(dir);
                        path = history.join("/");
                        let idx = arena_tree.node(&path);
                        current_idx = idx;
                    }
                },
                CommandLine::List => (),
                CommandLine::File { _name: _, size } => {
                    arena_tree.arena[current_idx].directory_size += size
                }
                CommandLine::Directory { name } => {
                    let new_path = match path.chars().last().unwrap() {
                        '/' => format!("{}{}", path, name),
                        _ => format!("{}/{}", path, name),
                    };
                    // let new_path = format!("{}{}", path, name);
                    let idx = arena_tree.node(&new_path);
                    arena_tree.arena[idx].parent = Some(current_idx); // set the child's parent
                    arena_tree.arena[current_idx].children.push(idx); // set the children
                }
            }
        }
        arena_tree
    }
    // Get index if it exists, if not insert it and get index
    fn node(&mut self, name: &str) -> usize {
        match self.arena.iter().find(|node| &node.name == name) {
            Some(node) => node.idx,
            None => {
                let idx = self.arena.len();
                self.arena.push(TreeNode::new(idx, name));
                idx
            }
        }
    }

    fn directories_with_at_most(self, limit: u32) -> u32 {
        self.arena
            .iter()
            .filter_map(|node| {
                let total_size = node.total_size(&self.arena);
                if total_size <= limit {
                    return Some(total_size);
                }
                None
            })
            .fold(0, |total, size| total + size)
    }

    fn need_to_delete(self, max: u32, needed: u32) -> u32 {
        let using = self.arena[0].total_size(&self.arena);
        println!("using: {}", using);
        let unused = max - using;
        println!("free: {}", unused);
        let goal = needed - unused;
        println!("min goal: {}", goal);

        self.arena
            .iter()
            .filter_map(|node| {
                let total_size = node.total_size(&self.arena);
                if total_size >= goal {
                    Some(total_size)
                } else {
                    None
                }
            })
            .reduce(
                |selected, node| {
                    if node < selected {
                        node
                    } else {
                        selected
                    }
                },
            )
            .unwrap()
    }

    // count all nodes
    fn _size(&self) -> usize {
        self.arena.len()
    }

    // count all edges (sum of every node's children)
    fn _edges(&self) -> usize {
        self.arena
            .iter()
            .fold(0, |acc, node| acc + node.children.len())
    }
}

#[derive(Debug)]
struct TreeNode {
    idx: usize,
    name: String,
    directory_size: u32,
    parent: Option<usize>,
    children: Vec<usize>,
}
impl TreeNode {
    fn new(idx: usize, name: &str) -> Self {
        Self {
            idx,
            name: name.to_string(),
            directory_size: 0,
            parent: None,
            children: vec![],
        }
    }

    fn total_size(&self, arena: &Vec<TreeNode>) -> u32 {
        // println!("{}", self.name);
        self.children
            .iter()
            .fold(self.directory_size, |total, idx| {
                let node = &arena[idx.to_owned()];
                total + node.total_size(arena)
            })
    }
}

fn parse_line(line: &str) -> CommandLine {
    let last = line.split_whitespace().last().unwrap();
    if line.starts_with("$ cd") {
        CommandLine::ChangeDirectory(last)
    } else if line.starts_with("$ ls") {
        CommandLine::List
    } else if line.starts_with("dir") {
        CommandLine::Directory { name: last }
    } else {
        let mut iter = line.split_whitespace();
        let size = iter.next().unwrap().parse::<u32>().unwrap();
        let name = iter.next().unwrap();
        CommandLine::File { _name: name, size }
    }
}

pub fn part1(contents: &str) -> u32 {
    let arena_tree = ArenaTree::parse(contents);
    arena_tree.directories_with_at_most(100000)
}

pub fn part2(contents: &str) -> u32 {
    let arena_tree = ArenaTree::parse(contents);
    arena_tree.need_to_delete(70000000, 30000000)
}
