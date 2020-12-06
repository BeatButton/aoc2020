use std::collections::HashSet;

const INPUT: &str = include_str!("input");

fn main() {
    let mut total = 0;
    for group in INPUT.split("\n\n") {
        let mut group = group.lines();
        let mut set: HashSet<char> = group.next().unwrap().chars().collect();
        for person in group {
            set = set
                .intersection(&person.chars().collect::<HashSet<char>>())
                .cloned()
                .collect()
        }
        total += set.len()
    }
    println!("{}", total);
}
