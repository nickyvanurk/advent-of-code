pub fn part1(input: &str) -> u32 {
    let mut node = Node::new("/".to_string(), 0);
    build_tree(&mut input.lines().rev().collect::<Vec<&str>>(), &mut node);

    node.get_dir_sizes().into_iter().filter(|&i| i <= 100000).sum()
}

pub fn part2(input: &str) -> u32 {
    let mut node = Node::new("/".to_string(), 0);
    build_tree(&mut input.lines().rev().collect::<Vec<&str>>(), &mut node);

    let min_size_dir_to_delete = 30000000 - (70000000 - node.used_space());
    node.get_dir_sizes().into_iter().filter(|&i| i >= min_size_dir_to_delete).min().unwrap()
}

fn build_tree(input: &mut Vec<&str>, node: &mut Node) {
    while let Some(line) = input.pop() {
        if !line.contains("$") {
            if line.contains("dir") {
                let directory_name = line.replace("dir ", "");
                node.add(Node::new(directory_name, 0));
            } else {
                let s = line.split(" ").collect::<Vec<&str>>();
                let (file_name, file_size) = (s[1], s[0].parse::<u32>().unwrap());
                node.add(Node::new(file_name.to_string(), file_size));
            }
        }

        if line.contains("$ cd") {
            if line.contains("..") {
                return;
            }

            let directory_name = line.replace("$ cd ", "");
            build_tree(input, node.get(directory_name).unwrap());
        }
    }
}

struct Node {
    name: String,
    size: u32,
    children: Vec<Node>,
}

impl Node {
    fn new(name: String, size: u32) -> Self {
        Node { name, size, children: vec![] }
    }

    fn add(&mut self, node: Node) {
        self.children.push(node);
    }

    fn get(&mut self, name: String) -> Option<&mut Node> {
        if self.name == name {
            return Some(self);
        }

        for node in &mut self.children {
            if node.name == name {
                return Some(node);
            }
        }

        None
    }

    fn get_dir_sizes(&self) -> Vec<u32> {
        let mut v = vec![];

        if self.is_directory() {
            v.push(self.used_space());
        }

        for node in &self.children {
            v.append(&mut node.get_dir_sizes());
        }

        v
    }

    fn used_space(&self) -> u32 {
        let mut size = self.size;

        for node in &self.children {
            size += node.used_space();
        }
     
        size
    }

    fn is_directory(&self) -> bool {
        self.size == 0
    }
}