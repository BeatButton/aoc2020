const INPUT: [[u8; 31]; 323] = include!("input");
const WIDTH: usize = INPUT[0].len();
const OVER: [usize; 5] = [1, 3, 5, 7, 1];
const DOWN: [usize; 5] = [1, 1, 1, 1, 2];

fn main() {
    println!(
        "{}",
        OVER.iter()
            .zip(&DOWN)
            .map(|(&over, &down)| {
                INPUT
                    .iter()
                    .step_by(down)
                    .zip((0_usize..).step_by(over))
                    .filter(|(line, idx)| line[idx % WIDTH] == b'#')
                    .count() as u64
            })
            .product::<u64>()
    );
}
