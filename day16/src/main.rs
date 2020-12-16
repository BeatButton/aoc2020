use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};

const INPUT: &str = include_str!("input");

fn main() {
    let mut iter = INPUT.lines();
    let fields = iter
        .by_ref()
        .take_while(|&line| !line.is_empty())
        .map(|line| {
            let mut split = line.split(": ");
            let name = split.next().unwrap();
            let ranges = split
                .next()
                .unwrap()
                .split(" or ")
                .map(|range| {
                    let mut range = range.split('-');
                    let start: u16 = range.next().unwrap().parse().unwrap();
                    let stop: u16 = range.next().unwrap().parse().unwrap();
                    start..=stop
                })
                .collect::<Vec<_>>();
            (name, ranges)
        })
        .collect::<HashMap<_, _>>();

    let _field_names = fields.keys().collect::<HashSet<_>>();

    let _my_ticket = iter
        .by_ref()
        .nth(1)
        .unwrap()
        .split(',')
        .map(str::parse::<u16>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut out = 0;

    for ticket in iter
        .skip(2)
        .map(|line| line.split(',').map(str::parse::<u16>).map(Result::unwrap))
    {
        for value in ticket {
            if !fields
                .values()
                .flatten()
                .any(|range| range.contains(&value))
            {
                out += value;
            }
        }
    }

    println!("{}", out)
}
