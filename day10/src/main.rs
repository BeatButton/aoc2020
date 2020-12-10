use lru::LruCache;

const INPUT: &str = include_str!("input");

fn fuck<'a>(adapters: &'a [u16], path: Vec<u16>, cache: &mut LruCache<&'a [u16], usize>) -> usize {
    if let Some(n) = cache.get(&adapters) {
        return *n;
    }
    if adapters.is_empty() {
        1
    } else {
        let prev = *path.last().unwrap();
        adapters
            .iter()
            .enumerate()
            .take_while(|(_, v)| (prev..=(prev + 3)).contains(v))
            .map(|(idx, &adapter)| {
                let mut path = path.clone();
                path.push(adapter);
                let adapts = &adapters[idx + 1..];
                let out = fuck(adapts, path, cache);
                cache.put(adapts, out);
                out
            })
            .sum()
    }
}

fn main() {
    let mut input: Vec<u16> = INPUT.lines().map(|line| line.parse().unwrap()).collect();
    input.sort_unstable();

    let mut cache = LruCache::new(1_000_000_000);

    let out = fuck(&input, vec![0], &mut cache);

    println!("{}", out);
}
