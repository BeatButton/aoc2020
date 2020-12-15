use fnv::FnvHashMap as HashMap;

const INPUT: &str = include_str!("input");
const TARGET: u64 = 30000000;

fn main() {
    let mut idx = 0;
    let mut seen: HashMap<u64, u64> = HashMap::default();
    for n in INPUT
        .trim_end()
        .split(',')
        .map(str::parse::<u64>)
        .map(Result::unwrap)
    {
        seen.insert(n, idx);
        idx += 1;
    }

    let mut curr = 0;
    let mut prev = 0;

    for idx in idx..TARGET {
        prev = curr;
        if let Some(past) = seen.get(&curr) {
            curr = idx - past;
        } else {
            curr = 0;
        }
        seen.insert(prev, idx);
    }
    println!("{}", prev);
}
