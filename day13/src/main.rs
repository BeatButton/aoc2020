const INPUT: &str = include_str!("input");

fn main() {
    let mut lines = INPUT.lines();
    let estimate: u32 = lines.next().unwrap().parse().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|time| time.parse::<u32>().ok())
        .collect::<Vec<_>>();

    println!(
        "{}",
        busses
            .iter()
            .map(|&bus| (bus, bus - (estimate - (estimate / bus) * bus)))
            .min_by_key(|&(_bus, wait)| wait)
            .map(|(bus, wait)| wait * bus)
            .unwrap()
    );
}
