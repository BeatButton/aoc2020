use std::hint::unreachable_unchecked;

use itertools::Itertools;

const INPUT: [u32; 200] = include!("input");

fn main() {
    for pair in INPUT.iter().combinations(3) {
        if let [&left, &middle, &right] = pair.as_slice() {
            if left + middle + right == 2020 {
                println!("{}", left * middle * right)
            }
        } else {
            unsafe { unreachable_unchecked() }
        }
    }
}
