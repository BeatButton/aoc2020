use itertools::Itertools;

const INPUT: &str = include_str!("input");

fn main() {
    println!(
        "{}",
        INPUT
            .lines()
            .map(|pass| {
                u16::from_str_radix(
                    pass.replace(['B', 'R'].as_ref(), "1")
                        .replace(['F', 'L'].as_ref(), "0")
                        .as_str(),
                    2,
                )
                .unwrap()
            })
            .sorted()
            .tuple_windows()
            .map(|(a, b)| (a + 1, b))
            .find(|(a, b)| a != b)
            .unwrap()
            .0
    );
}
