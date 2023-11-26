use super::Problem;
use std::collections::HashMap;
pub const P_07: Problem = Problem {
    number: 7,
    problem_a: a,
    problem_a_output: Some("1232307"),
    problem_b: b,
    problem_b_output: Some("7268994"),
};
#[derive(Clone, Debug)]
enum FileTreeNode {
    Directory {
        children: HashMap<String, FileTreeNode>,
        name: String,
        size: Option<u32>,
    },
    File {
        size: u32,
        name: String,
    },
}

impl FileTreeNode {
    fn get_name(&self) -> &str {
        match self {
            FileTreeNode::Directory { name, .. } => name,
            FileTreeNode::File { name, .. } => name,
        }
    }
    /// gets size of current node
    fn get_size(&self) -> u32 {
        match self {
            FileTreeNode::Directory { size, .. } => size.unwrap(),
            FileTreeNode::File { size, .. } => *size,
        }
    }
    fn populate_directory_size(&mut self) -> u32 {
        match self {
            FileTreeNode::Directory { children, size, .. } => {
                let children_size = children
                    .iter_mut()
                    .map(|(_name, child)| child.populate_directory_size())
                    .sum::<u32>();
                *size = Some(children_size);
                children_size
            }
            FileTreeNode::File { size, .. } => *size,
        }
    }
    // returns vec of all directory sizes
    fn get_directory_sizes(&self) -> Vec<u32> {
        match self {
            FileTreeNode::Directory { children, size, .. } => children
                .iter()
                .flat_map(|(_k, node)| node.get_directory_sizes())
                .chain(size.clone().into_iter())
                .collect(),
            FileTreeNode::File { .. } => Vec::new(),
        }
    }
    fn get_total_size_under_max(&self) -> u32 {
        let max_size = 100_000;

        match self {
            FileTreeNode::Directory { children, size, .. } => {
                let size = size.unwrap();
                let size = if size <= max_size { size } else { 0 };
                size + children
                    .iter()
                    .map(|(k, child)| child.get_total_size_under_max())
                    .sum::<u32>()
            }
            FileTreeNode::File { .. } => 0,
        }
    }
    fn add_directory(&mut self, path: &[String], mut files: HashMap<String, FileTreeNode>) {
        if path.len() == 0 {
            match self {
                FileTreeNode::Directory {
                    children,
                    name,
                    size,
                } => {
                    for (file_name, file) in files {
                        children.insert(file.get_name().to_string(), file);
                    }
                }
                FileTreeNode::File { .. } => panic!("can not add directory to regular file"),
            }
        } else {
            let child_name = &path[0];
            let new_path = path.iter().skip(1).cloned().collect::<Vec<_>>();
            match self {
                FileTreeNode::Directory {
                    children,
                    name,
                    size,
                } => {
                    for (_file_name, child) in children {
                        match child {
                            FileTreeNode::Directory { name, .. } => {
                                if name == child_name {
                                    return child.add_directory(&new_path, files);
                                }
                            }
                            FileTreeNode::File { .. } => {}
                        }
                    }
                }
                FileTreeNode::File { .. } => panic!("can not add directory to regular file"),
            }
        }
    }
}
#[derive(Clone, Debug)]
enum Command {
    ChangeDirectory {
        destination: String,
    },
    List {
        files: HashMap<String, FileTreeNode>,
    },
}
fn parse_command(input: &str) -> Vec<Command> {
    let mut in_ls = false;
    let mut list_files: HashMap<String, FileTreeNode> = HashMap::new();
    let mut commands = Vec::new();
    for line in input.lines().filter(|line| !line.is_empty()).skip(1) {
        let mut words = line.split_whitespace();
        let first_word = words.next();
        if first_word == Some("$") {
            if in_ls {
                commands.push(Command::List {
                    files: list_files.clone(),
                });
                list_files.clear();
            }
            let command_word = words.nth(0);
            if command_word == Some("cd") {
                in_ls = false;
                list_files.clear();

                commands.push(Command::ChangeDirectory {
                    destination: words.next().unwrap().to_string(),
                });
            } else if command_word == Some("ls") {
                in_ls = true;
                list_files.clear();
            } else {
                panic!("invalid command: \"{:#?}\"", command_word);
            }
        } else {
            if in_ls {
                if first_word.is_none() {
                    continue;
                }
                let first_word = first_word.unwrap();
                let second_word = words.next();
                if second_word.is_none() {
                    continue;
                }
                let second_word = second_word.unwrap();
                if first_word == "dir" {
                    list_files.insert(
                        second_word.to_string(),
                        FileTreeNode::Directory {
                            children: HashMap::new(),
                            name: second_word.to_string(),
                            size: None,
                        },
                    );
                } else {
                    list_files.insert(
                        first_word.parse().unwrap(),
                        FileTreeNode::File {
                            size: first_word.parse().unwrap(),
                            name: second_word.to_string(),
                        },
                    );
                }
            } else {
                panic!("not in ls")
            }
        };
    }
    if in_ls {
        commands.push(Command::List {
            files: list_files.clone(),
        });
    }
    commands
}
// need to build a tree
fn a(input: &str) -> String {
    let commands = parse_command(input);
    let mut file_tree = FileTreeNode::Directory {
        children: HashMap::new(),
        name: "".to_string(),
        size: None,
    };
    let mut current_path: Vec<String> = Vec::new();
    for command in commands.iter() {
        match command {
            Command::ChangeDirectory { destination } => {
                if destination == ".." {
                    current_path.pop();
                } else if destination == "." {
                    continue;
                } else {
                    current_path.push(destination.clone())
                }
            }
            Command::List { files } => file_tree.add_directory(&current_path, files.clone()),
        }
    }
    file_tree.populate_directory_size();
    //println!("file tree: {:#?}", file_tree);
    file_tree.get_total_size_under_max().to_string()
}
fn b(input: &str) -> String {
    // total size of file system
    let total_size = 70000000;
    let needed_size = 30000000;
    let commands = parse_command(input);
    let mut file_tree = FileTreeNode::Directory {
        children: HashMap::new(),
        name: "".to_string(),
        size: None,
    };
    let mut current_path: Vec<String> = Vec::new();
    for command in commands.iter() {
        match command {
            Command::ChangeDirectory { destination } => {
                if destination == ".." {
                    current_path.pop();
                } else if destination == "." {
                    continue;
                } else {
                    current_path.push(destination.clone())
                }
            }
            Command::List { files } => file_tree.add_directory(&current_path, files.clone()),
        }
    }
    file_tree.populate_directory_size();
    let used_size = file_tree.get_size();
    let remove_size = needed_size - (total_size - used_size);

    let mut sizes = file_tree.get_directory_sizes();
    sizes.sort();

    for s in sizes {
        if s >= remove_size {
            return s.to_string();
        }
    }

    String::new()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &'static str = r#"
$ cd /
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


    "#;
    #[test]
    fn one() {
        let r = a(TEST_INPUT);
        assert_eq!(&r, "95437")
    }
    #[test]
    fn test_b() {
        let r = b(TEST_INPUT);
        assert_eq!(&r, "24933642");
    }
}
