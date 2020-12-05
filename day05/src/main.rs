const INPUT: &str = include_str!("input");

fn main() {
    let mut ids: Vec<u16> = INPUT
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
        .collect();

    ids.sort_unstable();

    println!(
        "{}",
        ids.iter()
            .map(|&i| i + 1)
            .zip(ids.iter().skip(1))
            .find(|&(left, &right)| left != right)
            .unwrap()
            .0
    )
}
