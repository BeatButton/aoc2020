use std::collections::HashSet;

const INPUT: &str = include_str!("input");

fn main() {
    println!(
        "{}",
        INPUT
            .split("\n\n")
            .map(|group| group.lines())
            .map(|mut group| (group.next().unwrap().chars().collect::<HashSet<_>>(), group))
            .map(|(first, group)| {
                group
                    .fold(first, |set, person| {
                        set.intersection(&person.chars().collect::<HashSet<char>>())
                            .cloned()
                            .collect()
                    })
                    .len()
            })
            .sum::<usize>()
    );
}
