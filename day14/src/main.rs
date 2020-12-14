use itertools::Itertools;

use fnv::FnvHashMap as HashMap;

const INPUT: &str = include_str!("input");

fn main() {
    let mut one_mask = 0_u64;
    let mut floating_masks: Vec<u64> = vec![];
    let mut mem: HashMap<u64, u64> = HashMap::default();
    for line in INPUT.lines() {
        let mut iter = line.split(" = ");
        let instr = iter.next().unwrap();
        let value = iter.next().unwrap();
        if instr == "mask" {
            one_mask = value
                .chars()
                .rev()
                .enumerate()
                .map(|(idx, ch)| if ch == '1' { 2_u64.pow(idx as u32) } else { 0 })
                .sum();
            floating_masks.clear();
            floating_masks.extend(value.chars().rev().enumerate().filter_map(|(idx, ch)| {
                if ch == 'X' {
                    Some(2_u64.pow(idx as u32))
                } else {
                    None
                }
            }))
        } else {
            let mut addr = instr
                .chars()
                .skip(4)
                .take_while(|&ch| ch != ']')
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            let value = value.parse::<u64>().unwrap();
            addr |= one_mask;

            for &mask in &floating_masks {
                addr &= !mask;
            }

            for num_masks in 0..=floating_masks.len() {
                for masks in floating_masks.iter().cloned().combinations(num_masks) {
                    let a = masks.iter().fold(addr, |acc, &mask| acc | mask);

                    mem.insert(a, value);
                }
            }
        }
    }
    println!("{}", mem.values().sum::<u64>());
}
