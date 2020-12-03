const INPUT: &str = include_str!("input");

fn main() {
    let input: Vec<&str> = INPUT.lines().collect();
    let width = input[0].len();
    let over = 3;
    let down = 1;
    let mut y = 0;
    let mut x = 0;
    let mut num_trees = 0;
    while y < input.len() {
        if input[y].chars().nth(x % width).unwrap() == '#' {
            num_trees += 1;
        }
        y += down;
        x += over;
    }
    println!("{}", num_trees);
}
