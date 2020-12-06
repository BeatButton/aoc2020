use itertools::Itertools;

const INPUT: [u32; 200] = include!("input");

fn main() {
    println!(
        "{}",
        INPUT
            .iter()
            .tuple_combinations()
            .find(|&(a, b, c)| a + b + c == 2020)
            .map(|(a, b, c)| a * b * c)
            .unwrap()
    )
}
