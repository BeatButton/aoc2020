const INPUT: &str = include_str!("input");

fn main() -> anyhow::Result<()> {
    let mut num_valid = 0;
    for line in INPUT.lines() {
        let line = line;
        let mut iter = line.chars();
        let from: usize = iter
            .by_ref()
            .take_while(|&ch| ch != '-')
            .collect::<String>()
            .parse()?;
        let to: usize = iter
            .by_ref()
            .take_while(|&ch| ch != ' ')
            .collect::<String>()
            .parse()?;
        let character = iter.next().unwrap();
        let count = iter.filter(|&ch| ch == character).count();
        if from <= count && count <= to {
            num_valid += 1;
        }
    }
    println!("{}", num_valid);
    Ok(())
}
