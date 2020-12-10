use itertools::Itertools;

const INPUT: &str = include_str!("input");

fn fuck(adapters: &[u16], path: Vec<u16>) -> Option<Vec<u16>> {
    if adapters.is_empty() {
        return Some(path);
    }
    let prev = *path.last().unwrap();
    let limit = prev + 3;
    for (idx, &adapter) in adapters
        .iter()
        .enumerate()
        .take_while(|(_, v)| (prev..=limit).contains(v))
    {
        let mut path = path.clone();
        path.push(adapter);
        let left = &adapters[0..idx];
        let right = &adapters[idx + 1..];
        let adapters: Vec<u16> = left.iter().chain(right).cloned().collect();
        if let Some(out) = fuck(&adapters, path) {
            return Some(out);
        }
    }
    None
}

fn main() {
    let mut input: Vec<u16> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    input.sort_unstable();

    let path = fuck(&input, vec![0]).unwrap();

    let (one, three) =
        path.iter()
            .tuple_windows()
            .fold((0, 0), |(one, three), (from, to)| match to - from {
                1 => (one + 1, three),
                3 => (one, three + 1),
                _ => (one, three),
            });
    println!("{}", one * (three + 1));
}
