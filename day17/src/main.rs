use std::cmp;

use fnv::FnvHashSet as HashSet;

const INPUT: &str = include_str!("input");

type World = HashSet<(i64, i64, i64, i64)>;

fn num_active_neighbors(x: i64, y: i64, z: i64, w: i64, world: &World) -> usize {
    let mut count = 0;
    for x2 in (x - 1)..=(x + 1) {
        for y2 in (y - 1)..=(y + 1) {
            for z2 in (z - 1)..=(z + 1) {
                for w2 in (w - 1)..=(w + 1) {
                    if x2 == x && y2 == y && z2 == z && w2 == w {
                        continue;
                    }
                    if world.contains(&(x2, y2, z2, w2)) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn tick(world: &mut World) {
    let (mut max_x, mut max_y, mut max_z, mut max_w) = (0, 0, 0, 0);
    let (mut min_x, mut min_y, mut min_z, mut min_w) = (0, 0, 0, 0);
    let mut to_remove = vec![];
    let mut to_insert = vec![];
    for &(x, y, z, w) in world.iter() {
        max_x = cmp::max(max_x, x);
        max_y = cmp::max(max_y, y);
        max_z = cmp::max(max_z, z);
        max_w = cmp::max(max_w, w);
        min_x = cmp::min(min_x, x);
        min_y = cmp::min(min_y, y);
        min_z = cmp::min(min_z, z);
        min_w = cmp::min(min_w, w);
    }
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            for z in (min_z - 1)..=(max_z + 1) {
                for w in (min_w - 1)..=(max_w + 1) {
                    let count = num_active_neighbors(x, y, z, w, &world);
                    let active = world.contains(&(x, y, z, w));
                    if active && !(2..=3).contains(&count) {
                        to_remove.push((x, y, z, w));
                    } else if !active && count == 3 {
                        to_insert.push((x, y, z, w));
                    }
                }
            }
        }
    }
    for coord in to_remove {
        world.remove(&coord);
    }
    for coord in to_insert {
        world.insert(coord);
    }
}

fn main() {
    let mut world: World = INPUT
        .lines()
        .enumerate()
        .map(|(x, line)| {
            line.bytes()
                .enumerate()
                .filter(|&(_y, byte)| byte == b'#')
                .map(move |(y, _byte)| (x as i64, y as i64, 0, 0))
        })
        .flatten()
        .inspect(|a| println!("{:?}", a))
        .collect();

    for _ in 0..6 {
        println!("{}", world.len());
        tick(&mut world);
    }

    println!("{}", world.len());
}
