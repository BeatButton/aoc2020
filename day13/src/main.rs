use modinverse::modinverse;

const INPUT: &str = include_str!("input");

fn main() {
    let busses = INPUT
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(idx, time)| time.parse::<i64>().map(|time| (idx, time)).ok())
        .collect::<Vec<_>>();

    let m = busses.iter().map(|&(_idx, time)| time).product::<i64>();

    println!(
        "{}",
        busses
            .iter()
            .map(|&(idx, time)| {
                let b = m / time;
                (time - idx as i64) * b * modinverse(b, time).unwrap()
            })
            .sum::<i64>()
            % m
    )
}
