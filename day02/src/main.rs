const INPUT: &str = include_str!("input");

fn main() {
    println!(
        "{}",
        INPUT
            .lines()
            .map(|line| line.split('-'))
            .map(|mut split| (split.next().unwrap(), split.next().unwrap()))
            .map(|(idx1, line)| (idx1.parse::<usize>().unwrap(), line))
            .map(|(idx1, line)| {
                println!("{}", line);
                let mut line = line.chars();
                let idx2 = line
                    .by_ref()
                    .take_while(|&ch| ch != ' ')
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
                    - 1;
                let ch = line.next().unwrap();
                line.by_ref().take(2).for_each(|_| {});
                let password: String = line.collect();

                if (password.chars().nth(idx1).unwrap() == ch)
                    != (password.chars().nth(idx2).unwrap() == ch)
                {
                    1
                } else {
                    0
                }
            })
            .sum::<u32>()
    );
}
