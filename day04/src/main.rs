use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};

const INPUT: &str = include_str!("input");
const EXPECTED: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn main() {
    let expected: HashSet<String> = EXPECTED.iter().map(|s| s.to_string()).collect();
    let mut valid = 0;
    for passport in INPUT.split("\n\n") {
        let mut iter = passport.chars();
        let mut fields: HashMap<String, String> = HashMap::default();
        let mut labels: HashSet<String> = HashSet::default();
        loop {
            let label: String = iter.by_ref().take(3).collect();
            if label.is_empty() {
                break;
            }
            labels.insert(label.clone());
            let field = iter
                .by_ref()
                .skip(1)
                .take_while(|&ch| !ch.is_whitespace())
                .collect();
            fields.insert(label, field);
        }
        if expected.intersection(&labels).count() == 7 {
            if let Ok(true) = validate(fields) {
                valid += 1;
            }
        }
    }
    println!("{}", valid)
}

fn validate(fields: HashMap<String, String>) -> anyhow::Result<bool> {
    println!("{}", fields["byr"]);
    if !(1920..=2002).contains(&fields["byr"].parse::<u32>()?) {
        println!("byr not ok");
        return Ok(false);
    }
    if !(2010..=2020).contains(&fields["iyr"].parse::<u32>()?) {
        println!("iyr not ok");
        return Ok(false);
    }
    if !(2020..=2030).contains(&fields["eyr"].parse::<u32>()?) {
        println!("eyr not ok");
        return Ok(false);
    }
    let height = &fields["hgt"];
    let unit: String = height.chars().rev().take(2).collect();
    let num: u32 = height
        .chars()
        .take_while(|ch| ch.is_numeric())
        .collect::<String>()
        .parse()?;
    match unit.as_str() {
        "mc" => {
            if !(150..=193).contains(&num) {
                return Ok(false);
            }
        }
        "ni" => {
            if !(59..=76).contains(&num) {
                return Ok(false);
            }
        }
        _ => return Ok(false),
    }

    println!("hgt ok");
    let mut hair = fields["hcl"].chars();
    if let Some(ch) = hair.next() {
        if ch != '#' {
            return Ok(false);
        }
    } else {
        return Ok(false);
    };

    let color: String = hair.by_ref().take(6).collect();
    if color.len() < 6 {
        return Ok(false);
    }

    if color
        .chars()
        .any(|ch| !(ch.is_numeric() | ('a'..='f').contains(&ch)))
    {
        return Ok(false);
    }

    if hair.next().is_some() {
        return Ok(false);
    }
    println!("hcl ok");

    if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&fields["ecl"].as_str()) {
        return Ok(false);
    }

    println!("ecl ok");

    let pid = &fields["pid"];
    if pid.len() != 9 {
        return Ok(false);
    }

    if pid.chars().any(|ch| !ch.is_numeric()) {
        return Ok(false);
    }
    println!("pid ok");

    Ok(true)
}
