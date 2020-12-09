use std::cmp::Ordering;

use itertools::Itertools;

const INPUT: &str = include_str!("input");
const PREAMBLE_LEN: usize = 25;

fn main() {
    let data: Vec<u64> = INPUT
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let mut target = 0;
    for window in data.as_slice().windows(PREAMBLE_LEN + 1) {
        let i = window[PREAMBLE_LEN];
        let window = &window[..PREAMBLE_LEN];

        if !window
            .iter()
            .tuple_combinations()
            .map(|(a, b)| a + b)
            .any(|x| x == i)
        {
            target = i;
            break;
        }
    }

    let mut from = 0;
    let mut to = 0;
    let mut total = 0;
    loop {
        match total.cmp(&target) {
            Ordering::Greater => {
                total -= data[from];
                from += 1;
            }
            Ordering::Less => {
                total += data[to];
                to += 1;
            }
            Ordering::Equal => {
                let (min, max) = data[from..to].iter().minmax().into_option().unwrap();
                println!("{}", min + max);
                break;
            }
        }
    }
}
