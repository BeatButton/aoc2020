const INPUT: &str = include_str!("input");

fn angle(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (y2 - y1).atan2(x2 - x1)
}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x1 + x2).powi(2) + (y1 + y2).powi(2)).sqrt()
}

fn get_point(x: f64, y: f64, angle: f64, distance: f64) -> (f64, f64) {
    let x_mul = angle.cos();
    let y_mul = -angle.sin();
    (x + distance * x_mul, y + distance * y_mul)
}

fn main() {
    let mut way_x: f64 = 10.;
    let mut way_y: f64 = 1.;
    let mut ship_x: f64 = 0.;
    let mut ship_y: f64 = 0.;

    for line in INPUT.lines() {
        println!("{}: ({}, {}), ({}, {})", line, ship_x, ship_y, way_x, way_y);
        let instruction = &line[..1];
        let amount: f64 = line[1..].parse().unwrap();
        match instruction {
            "F" => {
                for _ in 0..(amount as usize) {
                    ship_x += way_x;
                    ship_y += way_y;
                }
            }
            "R" => match amount as usize {
                90 => {
                    let tmp = way_y;
                    way_y = -way_x;
                    way_x = tmp;
                }
                180 => {
                    way_x *= -1.;
                    way_y *= -1.;
                }
                270 => {
                    let tmp = way_x;
                    way_x = -way_y;
                    way_y = tmp;
                }
                _ => panic!("why"),
            },
            "L" => match amount as usize {
                90 => {
                    let tmp = way_x;
                    way_x = -way_y;
                    way_y = tmp;
                }
                180 => {
                    way_x *= -1.;
                    way_y *= -1.;
                }
                270 => {
                    let tmp = way_y;
                    way_y = -way_x;
                    way_x = tmp;
                }
                _ => panic!("why"),
            },

            "N" => way_y += amount,
            "E" => way_x += amount,
            "S" => way_y -= amount,
            "W" => way_x -= amount,
            _ => panic!("unrecognized {}", instruction),
        }
    }
    println!("{}", ship_x.abs() + ship_y.abs());
}
