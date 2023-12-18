use std::{fs, usize};

pub struct Node {
    pub(crate) name: String,
    targets: (String, String),
}

impl Node {
    fn from_string(string: &str) -> Node {
        let mut split = string.split(" = ");

        let name = split.next().unwrap().to_string();
        let next_part = split.next().unwrap().replace("(", "").replace(")", "");
        let mut target_split =  next_part.split(", ");

        let left = target_split.next().unwrap().to_string();
        let right = target_split.next().unwrap().to_string();


        return Node {
            name,
            targets: (left, right),
        };
    }
    pub(crate) fn next_node<'a>(&self, instruction: char, nodes: &'a Vec<Node>) -> &'a Node {
        let node_name: &String = if instruction == 'L' { &self.targets.0 } else { &self.targets.1 };

        return nodes.iter().find(|node| node.name == *node_name).unwrap();
    }
}

pub fn parse_data(data: String) -> (Vec<char>, Vec<Node>) {
    let mut lines = data.lines();

    let instructions: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next();

    let nodes: Vec<Node> = lines.map(Node::from_string).collect();

    return (instructions, nodes);
}

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let (instructions, nodes) = parse_data(data);

    let mut node = nodes.iter().find(|node| node.name == "AAA").unwrap().clone();
    let mut i: usize = 0;

    while node.name != "ZZZ" {
        node = node.next_node(instructions[i % instructions.len()], &nodes);
        i += 1;
    }

    println!("{}", i);
}
