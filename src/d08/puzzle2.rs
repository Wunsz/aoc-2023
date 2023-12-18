use std::{fs, usize};
use crate::d08::puzzle1::{Node, parse_data};

pub fn run(input_file: String) {
    let data = fs::read_to_string(input_file)
        .expect("Should have been able to read the file");

    let (instructions, nodes) = parse_data(data);

    let mut current_nodes: Vec<&Node> = nodes.iter().filter(|node| node.name.ends_with("A")).collect();
    let mut i: usize = 0;
    let mut result_steps: Vec<i64> = vec![];

    while current_nodes.iter().any(|node| !node.name.ends_with("Z")) {
        for node_index in 0..current_nodes.len() {
            if current_nodes[node_index].name.ends_with("Z") {
                continue;
            }

            current_nodes[node_index] = current_nodes[node_index].next_node(instructions[i % instructions.len()], &nodes);

            if current_nodes[node_index].name.ends_with("Z") {
                result_steps.push(i as i64 + 1);
            }
        }

        i += 1;
    }

    println!("{}", result_steps.iter().fold(1, |x, y| num_integer::lcm(x, *y)));
}
