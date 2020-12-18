use std::{cmp, ops::RangeInclusive};

use fnv::FnvHashSet as HashSet;
use itertools::iproduct;

const INPUT: &str = include_str!("input");

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl From<(i64, i64, i64, i64)> for Point {
    fn from(point: (i64, i64, i64, i64)) -> Self {
        let (x, y, z, w) = point;
        Self { x, y, z, w }
    }
}

impl Point {
    fn dimmax(self, other: Self) -> Self {
        let Self {
            x: x1,
            y: y1,
            z: z1,
            w: w1,
        } = self;
        let Self {
            x: x2,
            y: y2,
            z: z2,
            w: w2,
        } = other;
        let x = cmp::max(x1, x2);
        let y = cmp::max(y1, y2);
        let z = cmp::max(z1, z2);
        let w = cmp::max(w1, w2);
        Self { x, y, z, w }
    }

    fn dimmin(self, other: Self) -> Self {
        let Self {
            x: x1,
            y: y1,
            z: z1,
            w: w1,
        } = self;
        let Self {
            x: x2,
            y: y2,
            z: z2,
            w: w2,
        } = other;
        let x = cmp::min(x1, x2);
        let y = cmp::min(y1, y2);
        let z = cmp::min(z1, z2);
        let w = cmp::min(w1, w2);
        Self { x, y, z, w }
    }
}

type Points = HashSet<Point>;

struct World {
    points: Points,
    min: Point,
    max: Point,
}

impl From<Points> for World {
    fn from(points: Points) -> Self {
        let mut max: Point = Default::default();
        let mut min: Point = Default::default();

        for &point in &points {
            min = min.dimmin(point);
            max = max.dimmax(point);
        }

        Self { points, min, max }
    }
}

fn adjacent(n: i64) -> RangeInclusive<i64> {
    (n - 1)..=(n + 1)
}

fn num_active_neighbors(point: Point, points: &Points) -> usize {
    let Point { x, y, z, w } = point;
    iproduct!(adjacent(x), adjacent(y), adjacent(z), adjacent(w))
        .map(Into::into)
        .filter(|point| points.contains(point))
        .count()
        - points.contains(&point) as usize
}

fn tick(world: &mut World) {
    let (remove, insert) = iproduct!(
        (world.min.x - 1)..=(world.max.x + 1),
        (world.min.y - 1)..=(world.max.y + 1),
        (world.min.z - 1)..=(world.max.z + 1),
        (world.min.w - 1)..=(world.max.w + 1)
    )
    .map(Into::into)
    .fold((vec![], vec![]), |(mut remove, mut insert), point| {
        let count = num_active_neighbors(point, &world.points);
        let active = world.points.contains(&point);
        if active && !(2..=3).contains(&count) {
            remove.push(point);
        } else if !active && count == 3 {
            insert.push(point);
        }
        (remove, insert)
    });

    for coord in remove {
        world.points.remove(&coord);
    }
    for coord in insert {
        world.points.insert(coord);
        world.min = world.min.dimmin(coord);
        world.max = world.max.dimmax(coord);
    }
}

fn main() {
    let mut world = World::from(
        INPUT
            .lines()
            .enumerate()
            .map(|(x, line)| {
                line.bytes()
                    .enumerate()
                    .filter(|&(_y, byte)| byte == b'#')
                    .map(move |(y, _byte)| (x as i64, y as i64, 0, 0))
            })
            .flatten()
            .map(Into::into)
            .collect::<Points>(),
    );

    for _ in 0..6 {
        tick(&mut world);
    }

    println!("{}", world.points.len());
}
