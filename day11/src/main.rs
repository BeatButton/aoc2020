const INPUT: &str = include_str!("input");

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Floor,
    Seat(State),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum State {
    Empty,
    Occupied,
}

use Cell::*;
use State::*;

impl Cell {
    fn new(ch: char) -> Self {
        match ch {
            '.' => Floor,
            'L' => Seat(Empty),
            '#' => Seat(Occupied),
            other => panic!("Invalid cell: {}", other),
        }
    }
}

fn tick(seats: &[Vec<Cell>]) -> Vec<Vec<Cell>> {
    let mut out: Vec<Vec<Cell>> = seats.iter().map(|row| row.to_vec()).collect();
    for (r, row) in seats.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            let num_occupied = if let Seat(_) = cell {
                get_num_occupied_seats(&seats, r as isize, c as isize)
            } else {
                continue;
            };
            /*println!("{}, {} => {}", r, c, num_occupied,);*/
            match cell {
                Seat(Empty) => {
                    if num_occupied == 0 {
                        out[r][c] = Seat(Occupied);
                    }
                }
                Seat(Occupied) => {
                    if num_occupied >= 5 {
                        out[r][c] = Seat(Empty);
                    }
                }
                Floor => unreachable!(),
            }
        }
    }
    out
}

fn get_num_occupied_seats(cells: &[Vec<Cell>], row: isize, col: isize) -> usize {
    let mut out = 0;
    let num_rows = cells.len();
    let num_cols = cells[0].len();
    for &(dc, dr) in &DIRECTIONS {
        let mut r = row;
        let mut c = col;
        loop {
            r += dr;
            c += dc;
            if r < 0 || r as usize >= num_rows || c < 0 || c as usize >= num_cols {
                break;
            }
            if let Seat(is_occupied) = cells[r as usize][c as usize] {
                if let Occupied = is_occupied {
                    out += 1;
                }
                break;
            }
        }
    }
    out
}

fn main() {
    let mut seats: Vec<Vec<Cell>> = INPUT
        .lines()
        .map(|line| line.chars().map(Cell::new).collect())
        .collect();

    loop {
        /*println!("{}", i);
        for row in &seats {
            for cell in row {
                print!(
                    "{}",
                    match cell {
                        Seat(Occupied) => '#',
                        Seat(Empty) => 'L',
                        Floor => '.',
                    }
                );
            }
            println!();
        }*/
        let next = tick(seats.as_slice());
        if next == seats {
            println!(
                "{}",
                next.iter()
                    .map(|row| row.iter().filter(|&&cell| cell == Seat(Occupied)).count())
                    .sum::<usize>()
            );
            break;
        } else {
            seats = next;
        }
    }
}
