const INPUT: &str = include_str!("input");
const OVER: [usize; 5] = [1, 3, 5, 7, 1];
const DOWN: [usize; 5] = [1, 1, 1, 1, 2];

fn main() {
    let input: Vec<&str> = INPUT.lines().collect();
    let width = input[0].len();
    let mut out: u64 = 1;
    for (over, down) in OVER.iter().zip(&DOWN) {
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
        out *= num_trees;
    }
    println!("{}", out);
}
