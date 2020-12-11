use std::cmp::max;

const INPUT: &str = include_str!("input");

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
            let adjacent = if let Seat(_) = cell {
                get_adjacent_cells(&seats, r, c)
            } else {
                continue;
            };
            /*println!(
                "At ({}, {}) there were {} adjacent cells",
                r,
                c,
                adjacent.len()
            );*/
            let num_occupied: usize = adjacent
                .into_iter()
                .map(|cell| if let Seat(Occupied) = cell { 1 } else { 0 })
                .sum();
            match cell {
                Seat(Empty) => {
                    if num_occupied == 0 {
                        out[r][c] = Seat(Occupied);
                    }
                }
                Seat(Occupied) => {
                    if num_occupied >= 4 {
                        out[r][c] = Seat(Empty);
                    }
                }
                Floor => unreachable!(),
            }
        }
    }
    out
}

fn get_adjacent_cells(cells: &[Vec<Cell>], row_idx: usize, col_idx: usize) -> Vec<Cell> {
    let row_min = max(row_idx, 1) - 1;
    let row_take = if row_idx == 0 { 2 } else { 3 };
    let col_min = max(col_idx, 1) - 1;
    let col_take = if col_idx == 0 { 2 } else { 3 };

    cells
        .iter()
        .enumerate()
        .skip(row_min)
        .take(row_take)
        .map(|(r, row)| {
            row.iter()
                .enumerate()
                .skip(col_min)
                .take(col_take)
                .filter(move |&(c, _cell)| c != col_idx || r != row_idx)
                .map(|(_c, &cell)| cell)
        })
        .flatten()
        .collect()
}

fn main() {
    let mut seats: Vec<Vec<Cell>> = INPUT
        .lines()
        .map(|line| line.chars().map(Cell::new).collect())
        .collect();

    for i in 0.. {
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
