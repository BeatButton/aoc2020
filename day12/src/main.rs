const INPUT: &str = include_str!("input");

fn main() {
    let mut way_x: i64 = 10;
    let mut way_y: i64 = 1;
    let mut ship_x: i64 = 0;
    let mut ship_y: i64 = 0;

    for line in INPUT.lines() {
        let instruction = &line[..1];
        let amount: i64 = line[1..].parse().unwrap();
        match instruction {
            "F" => {
                ship_x += way_x * amount;
                ship_y += way_y * amount;
            }
            "L" | "R" => {
                let flip = if instruction == "L" { 1 } else { -1 };
                match amount {
                    90 => {
                        let tmp = way_y;
                        way_y = flip * way_x;
                        way_x = -flip * tmp;
                    }
                    180 => {
                        way_x *= -1;
                        way_y *= -1;
                    }
                    270 => {
                        let tmp = way_x;
                        way_x = flip * way_y;
                        way_y = -flip * tmp;
                    }
                    _ => panic!(":("),
                }
            }
            "N" => way_y += amount,
            "E" => way_x += amount,
            "S" => way_y -= amount,
            "W" => way_x -= amount,
            _ => panic!("unrecognized {}", instruction),
        }
    }
    println!("{}", ship_x.abs() + ship_y.abs());
}
