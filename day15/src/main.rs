use fnv::FnvHashMap as HashMap;

const INPUT: &str = include_str!("input");
const TARGET: u64 = 30000000;

fn main() {
    let mut seen: HashMap<u64, u64> = INPUT
        .trim_end()
        .split(',')
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .enumerate()
        .map(|(idx, k)| (k, idx as _))
        .collect();

    println!(
        "{}",
        ((seen.len() as _)..(TARGET - 1)).fold(0, |prev, idx| {
            let curr = if let Some(past) = seen.get(&prev) {
                idx - past
            } else {
                0
            };
            seen.insert(prev, idx);
            curr
        })
    );
}
