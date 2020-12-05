use std::collections::HashSet;

const INPUT: &str = include_str!("input");

fn main() {
    let mut set: HashSet<u16> = (0..=1023).collect();
    for pass in INPUT.lines() {
        let mut iter = pass.chars();
        let row = u16::from_str_radix(
            iter.by_ref()
                .take(7)
                .map(|ch| match ch {
                    'B' => '1',
                    'F' => '0',
                    _ => panic!(),
                })
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap();
        let col = u16::from_str_radix(
            iter.take(3)
                .map(|ch| match ch {
                    'R' => '1',
                    'L' => '0',
                    _ => '_',
                })
                .collect::<String>()
                .as_str(),
            2,
        )
        .unwrap();
        let id = row * 8 + col;
        set.remove(&id);
    }
    let mut i = 0;
    while set.remove(&i) {
        i += 1;
    }
    i = 1023;
    while set.remove(&i) {
        i -= 1;
    }
    println!("{}", set.iter().next().unwrap());
}
