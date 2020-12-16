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
                    let start: u64 = range.next().unwrap().parse().unwrap();
                    let stop: u64 = range.next().unwrap().parse().unwrap();
                    start..=stop
                })
                .collect::<Vec<_>>();
            (name, ranges)
        })
        .collect::<HashMap<_, _>>();

    let field_names = fields.keys().collect::<HashSet<_>>();

    let my_ticket = iter
        .by_ref()
        .nth(1)
        .unwrap()
        .split(',')
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let mut possible_fields = vec![field_names; my_ticket.len()];

    for ticket in iter
        .skip(2)
        .map(|line| {
            line.split(',')
                .map(str::parse::<u64>)
                .map(Result::unwrap)
                .collect::<Vec<_>>()
        })
        .filter(|ticket| {
            ticket
                .iter()
                .all(|value| fields.values().flatten().any(|range| range.contains(value)))
        })
    {
        for (value, possible_fields) in ticket.into_iter().zip(possible_fields.iter_mut()) {
            for field in possible_fields.clone() {
                if !fields
                    .get(field)
                    .unwrap()
                    .iter()
                    .any(|range| range.contains(&value))
                {
                    possible_fields.remove(field);
                }
            }
        }
    }

    loop {
        let mut changed = false;
        for fields in possible_fields.clone() {
            if fields.len() == 1 {
                let field = fields.iter().next().unwrap();
                for possibilities in possible_fields.iter_mut() {
                    if possibilities.len() == 1 {
                        continue;
                    }
                    let removed = possibilities.remove(field);
                    changed |= removed;
                }
            }
        }

        if !changed {
            break;
        }
    }

    let out = my_ticket
        .into_iter()
        .zip(possible_fields)
        .map(|(value, fields)| {
            println!("{:?}", fields);
            if fields.into_iter().next().unwrap().starts_with("departure") {
                value
            } else {
                1
            }
        })
        .product::<u64>();

    println!("{}", out);
}
