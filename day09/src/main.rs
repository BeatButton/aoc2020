use itertools::Itertools;

const INPUT: &str = include_str!("input");
const PREAMBLE_LEN: usize = 25;

fn main() {
    let data: Vec<u64> = INPUT
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect();

    let mut invalid = 0;
    for window in data.as_slice().windows(PREAMBLE_LEN + 1) {
        let i = window[PREAMBLE_LEN];
        let window = &window[..PREAMBLE_LEN];

        if !window
            .iter()
            .tuple_combinations()
            .map(|(a, b)| a + b)
            .any(|x| x == i)
        {
            invalid = i;
            break;
        }
    }

    for (i, &start) in data.iter().enumerate() {
        let mut min = start;
        let mut max = start;
        let mut total = start;
        for &next in &data[i + 1..] {
            total += next;
            min = std::cmp::min(next, min);
            max = std::cmp::max(next, max);
            if total >= invalid {
                break;
            }
        }
        if total == invalid {
            println!("{}", min + max);
        }
    }
}
