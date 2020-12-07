use std::vec;

use petgraph::graphmap::DiGraphMap;

const INPUT: &str = include_str!("input");

fn main() {
    let mut graph: DiGraphMap<&str, usize> = DiGraphMap::new();
    for line in INPUT.lines() {
        let mut split = line[..line.len() - 1].split(" contain ");
        let key = split.next().unwrap().trim_end_matches('s');
        let values = split.next().unwrap();
        graph.add_node(key);
        if values == "no other bags" {
            continue;
        }
        for value in values.split(", ") {
            let amount: usize = value[0..1].parse().unwrap();
            let value = &value[2..].trim_end_matches('s');

            graph.add_node(value);
            graph.add_edge(key, value, amount);
        }
    }

    let mut out = 0;
    let mut stack: Vec<&str> = vec!["shiny gold bag"];
    while let Some(bag) = stack.pop() {
        out += 1;
        for (_, next, &amount) in graph.edges(bag) {
            stack.extend(vec![next; amount])
        }
    }
    out -= 1;

    println!("{}", out);
}
