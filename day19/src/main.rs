use fnv::FnvHashMap as HashMap;

const INPUT: &str = include_str!("test");

fn matches(rules: &HashMap<usize, &str>, text: &str) -> bool {
    let (matched, length) = matches_impl(0, rules, 0, text);
    matched && length == text.len()
}

fn matches_impl(
    rule_idx: usize,
    rules: &HashMap<usize, &str>,
    mut text_idx: usize,
    text: &str,
) -> (bool, usize) {
    //    println!("applying rule {} to {}", rules[rule_idx], &text[text_idx..]);
    if text_idx == text.len() {
        return (false, 0);
    }
    let expr = rules.get(&rule_idx).unwrap();
    if expr.starts_with('"') {
        (text[text_idx..text_idx + 1] == expr[1..2], 1)
    } else {
        for subexpr in expr.split(" | ") {
            let mut total_delta = 0;
            let mut matched = true;
            for rule_idx in subexpr.split(' ') {
                if let (true, delta) =
                    matches_impl(rule_idx.parse::<usize>().unwrap(), rules, text_idx, text)
                {
                    text_idx += delta;
                    total_delta += delta;
                } else {
                    matched = false;
                    break;
                }
            }
            if matched {
                return (true, total_delta);
            }
            text_idx -= total_delta;
        }
        (false, 0)
    }
}

pub fn main() {
    let mut lines = INPUT.lines();
    let rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let idx = line
                .bytes()
                .enumerate()
                .find(|&(_idx, b)| b == b' ')
                .unwrap()
                .0;
            (line[..idx - 1].parse::<usize>().unwrap(), &line[idx + 1..])
        })
        .collect::<HashMap<_, _>>();

    let out = lines
        .filter(|line| matches(&rules, line))
        .inspect(|line| println!("{}", line))
        .count();

    println!("{}", out);
}
