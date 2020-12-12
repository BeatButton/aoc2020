const INPUT: &str = include_str!("input");

fn main() {
    let mut x: f64 = 0.;
    let mut y: f64 = 0.;
    let mut degrees: f64 = 0.; // due east

    for line in INPUT.lines() {
        let instruction = &line[..1];
        let amount: f64 = line[1..].parse().unwrap();
        match instruction {
            "F" => {
                let rads = degrees * std::f64::consts::TAU / 360.;
                let x_mul = rads.cos();
                let y_mul = -rads.sin();
                x += amount * x_mul;
                y += amount * y_mul;
            }
            "R" => {
                degrees += amount;
                degrees %= 360.;
            }
            "L" => {
                degrees -= amount;
                degrees %= 360.;
            }

            "N" => y += amount,
            "E" => x += amount,
            "S" => y -= amount,
            "W" => x -= amount,
            _ => panic!("unrecognized {}", instruction),
        }
    }
    println!("{}", x.abs() + y.abs());
}
