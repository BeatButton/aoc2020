use fnv::FnvHashMap as HashMap;

const INPUT: &str = include_str!("input");

fn solve(idx: usize, adapters: &[u16], seen: &mut HashMap<usize, u64>) -> u64 {
    if let Some(&n) = seen.get(&idx) {
        n
    } else if idx == adapters.len() - 1 {
        1
    } else {
        adapters[idx..]
            .iter()
            .enumerate()
            .skip(1)
            .take_while(|(_, &v)| v <= adapters[idx] + 3)
            .map(|(i, _)| {
                let arg = i + idx;
                let out = solve(arg, adapters, seen);
                seen.insert(arg, out);
                out
            })
            .sum()
    }
}

fn main() {
    let mut input: Vec<u16> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    input.push(0);
    input.sort_unstable();
    let mut seen: HashMap<usize, u64> = HashMap::default();

    let out = solve(0, &input, &mut seen);

    println!("{}", out);
}
