use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Command {
    List(Vec<(usize, String)>),
    ChangeDir(String),
}

type NodeIndex = usize;

#[derive(Debug, Clone)]
enum Entry {
    File(usize),
    Dir(usize, HashMap<String, NodeIndex>),
}

#[derive(Debug)]
struct Tree {
    nodes: Vec<Entry>,
}

fn main() {
    let input = BufReader::new(File::open("input.txt").unwrap());
    let cmds = parse_commands(input);
    let mut tree = make_tree(&cmds);
    calculate_dir_sizes(&mut tree);
    let below100k = below_100k(&tree);
    let smal = smallest_to_remove(&tree);
    print_tree(&tree);
    println!("below 100k: {}", below100k);
    dbg!(smal);
}

fn make_tree(cmds: &[Command]) -> Tree {
    let mut tree = Tree {
        nodes: vec![Entry::Dir(0, HashMap::new())],
    };
    let mut path = vec![0];

    // first command will just be 'cd /'
    let cmds = &cmds[1..];

    for cmd in cmds {
        match cmd {
            Command::List(entries) => {
                for (size, name) in entries {
                    let new_ind = tree.nodes.len();

                    match &mut tree.nodes[*path.last().unwrap()] {
                        Entry::File(_) => panic!(),
                        Entry::Dir(_, m) => {
                            m.insert(name.to_string(), new_ind);
                            tree.nodes.push(Entry::File(*size));
                        }
                    }
                }
            }
            Command::ChangeDir(dir) => {
                if dir == ".." {
                    path.pop();
                } else {
                    let new_ind = tree.nodes.len();

                    match &mut tree.nodes[*path.last().unwrap()] {
                        Entry::File(_) => panic!(),
                        Entry::Dir(_, m) => {
                            m.insert(dir.to_owned(), new_ind);
                            path.push(new_ind);
                            tree.nodes.push(Entry::Dir(0, HashMap::new()));
                        }
                    }
                }
            }
        }
    }

    tree
}

fn parse_commands(r: impl BufRead) -> Vec<Command> {
    let mut cmds = vec![];
    let mut it = r.lines().flatten().peekable();

    while let Some(line) = it.next() {
        if line == "$ ls" {
            let mut entries = vec![];
            while let Some(p) = it.peek() {
                if !p.starts_with('$') {
                    let entry = it.next().unwrap();
                    let (size_or_type, name) = entry.split_once(' ').unwrap();
                    if let Ok(size) = size_or_type.parse::<usize>() {
                        entries.push((size, name.to_owned()));
                    }
                } else {
                    break;
                }
            }
            cmds.push(Command::List(entries));
        } else if let Some(dir) = line.strip_prefix("$ cd ") {
            cmds.push(Command::ChangeDir(dir.to_owned()));
        }
    }

    cmds
}

fn print_tree_inner(tree: &Tree, entries: &HashMap<String, NodeIndex>, indent: usize) {
    for (name, ind) in entries {
        let node = &tree.nodes[*ind];
        match node {
            Entry::File(size) => println!("{:indent$}{} {}", "", size, name, indent = indent),
            Entry::Dir(size, entries) => {
                println!("{:indent$}{} {}/", "", size, name, indent = indent);
                // push all entries? not a path, its a processing queue.
                print_tree_inner(tree, entries, indent + 4)
            }
        }
    }
}

fn print_tree(tree: &Tree) {
    let root = &tree.nodes[0];

    match root {
        Entry::File(_) => todo!(),
        Entry::Dir(size, entries) => {
            print_tree_inner(tree, entries, 0);
            println!("Total size: {}", size);
        }
    }
}

fn calculate_dir_sizes(tree: &mut Tree) {
    fn inner(tree: &mut Tree, current: NodeIndex, entries: HashMap<String, NodeIndex>) -> usize {
        let mut dir_size = 0;

        for (_, ind) in entries {
            let node = &tree.nodes[ind];
            match node {
                Entry::File(size) => dir_size += size,
                Entry::Dir(_, ents) => dir_size += inner(tree, ind, ents.clone()),
            }
        }

        // dir size for the current directory, including subdirs. What node is
        // this directory, so we can set it through the tree?
        match &mut tree.nodes[current] {
            Entry::File(_) => todo!(),
            Entry::Dir(e, _) => *e = dir_size,
        }

        dir_size
    }

    let mut root = tree.nodes[0].clone();
    match &mut root {
        Entry::File(_) => todo!(),
        Entry::Dir(e, entries) => *e = inner(tree, 0, entries.clone()),
    };
}

fn below_100k(tree: &Tree) -> usize {
    fn inner(tree: &Tree, current: NodeIndex, entries: HashMap<String, NodeIndex>) -> usize {
        let mut count = 0;

        for (_, ind) in entries {
            let node = &tree.nodes[ind];
            match node {
                Entry::File(_) => {}
                Entry::Dir(_, ents) => count += inner(tree, ind, ents.clone()),
            }
        }

        // dir size for the current directory, including subdirs. What node is
        // this directory, so we can set it through the tree?
        match &tree.nodes[current] {
            Entry::File(_) => todo!(),
            Entry::Dir(e, _) => {
                if *e <= 100_000 {
                    count += *e
                }
            }
        }

        count
    }

    let mut root = tree.nodes[0].clone();
    match &mut root {
        Entry::File(_) => todo!(),
        Entry::Dir(e, entries) => {
            let count = inner(tree, 0, entries.clone());
            if *e <= 100_000 {
                count + *e
            } else {
                count
            }
        }
    }
}

const TOTAL_USED: usize = 42_476_859;
const TOTAL_CAP: usize = 70_000_000;
const TOTAL_NEED_FREE: usize = 30_000_000;
const SMALLEST: usize = TOTAL_USED - (TOTAL_CAP - TOTAL_NEED_FREE);

fn smallest_to_remove(tree: &Tree) -> usize {
    let smallest_found = usize::MAX;

    fn inner(
        tree: &Tree,
        mut smallest_found: usize,
        current: NodeIndex,
        entries: HashMap<String, NodeIndex>,
    ) -> usize {
        for (_, ind) in entries {
            let node = &tree.nodes[ind];
            match node {
                Entry::File(_) => {}
                Entry::Dir(_, ents) => {
                    smallest_found = inner(tree, smallest_found, ind, ents.clone())
                }
            }
        }

        // dir size for the current directory, including subdirs. What node is
        // this directory, so we can set it through the tree?
        match &tree.nodes[current] {
            Entry::File(_) => todo!(),
            Entry::Dir(e, _) => {
                if *e < smallest_found && *e >= SMALLEST {
                    smallest_found = *e
                }
            }
        }

        smallest_found
    }

    let mut root = tree.nodes[0].clone();
    match &mut root {
        Entry::File(_) => todo!(),
        Entry::Dir(e, entries) => {
            let smallest_found = inner(tree, smallest_found, 0, entries.clone());
            if *e < smallest_found && *e >= SMALLEST {
                *e
            } else {
                smallest_found
            }
        }
    }
}

#[cfg(test)]
mod test {

    use std::io::Cursor;

    use crate::{below_100k, calculate_dir_sizes, make_tree, parse_commands, print_tree, Command};
    const INPUT: &str = "$ cd /
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
7214296 k
";

    #[test]
    fn parse() {
        let cmds = parse_commands(Cursor::new(INPUT));
        assert!(matches!(cmds[1], Command::List(_)));
    }

    #[test]
    fn to_tree() {
        let cmds = parse_commands(Cursor::new(INPUT));
        let tree = make_tree(&cmds);
        print_tree(&tree);
    }

    #[test]
    fn calc() {
        let cmds = parse_commands(Cursor::new(INPUT));
        let mut tree = make_tree(&cmds);
        calculate_dir_sizes(&mut tree);
        let below100k = below_100k(&tree);
        print_tree(&tree);
        println!("below 100k: {}", below100k);
    }
}
