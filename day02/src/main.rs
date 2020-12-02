const INPUT: &str = include_str!("input");

fn main() -> anyhow::Result<()> {
    let mut num_valid = 0;
    for line in INPUT.lines() {
        let line = line;
        let mut iter = line.chars();
        let idx1 = iter
            .by_ref()
            .take_while(|&ch| ch != '-')
            .collect::<String>()
            .parse::<usize>()?
            - 1;
        let idx2 = iter
            .by_ref()
            .take_while(|&ch| ch != ' ')
            .collect::<String>()
            .parse::<usize>()?
            - 1;
        let ch = iter.next().unwrap();
        iter.by_ref().take(2).for_each(|_| {});
        let password: String = iter.collect();

        if (password.chars().nth(idx1).unwrap() == ch)
            != (password.chars().nth(idx2).unwrap() == ch)
        {
            num_valid += 1;
        }
    }
    println!("{}", num_valid);
    Ok(())
}
