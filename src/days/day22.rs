use std::collections::{HashMap, HashSet};

use crate::harness::input::RawInput;

type Pos3 = (usize, usize, usize);
type Pos2 = (usize, usize);

pub fn solve_part1(input: RawInput) -> usize {
    let pile = build_pile(input);
    let mut must_keep = HashSet::<usize>::new();
    for brick in &pile {
        if brick.below.len() == 1 {
            must_keep.insert(*brick.below.iter().next().unwrap());
        }
    }
    pile.len() - must_keep.len()
}

pub fn solve_part2(input: RawInput) -> usize {
    let pile = build_pile(input);
    let mut count = 0;
    for i in 0..pile.len() {
        let mut lost_supports = vec![0_usize; pile.len()];
        let mut pending = vec![i];
        while let Some(brick_index) = pending.pop() {
            for &above in &pile[brick_index].above {
                lost_supports[above] += 1;
                if lost_supports[above] == pile[above].below.len() {
                    pending.push(above);
                    count += 1;
                }
            }
        }
    }
    count
}

#[derive(Debug, Default)]
struct BrickInfo {
    below: HashSet<usize>,
    above: HashSet<usize>,
}

fn build_pile(input: RawInput) -> Vec<BrickInfo> {
    let mut bricks = input
        .per_line(|line| parse_brick(line.as_str()))
        .collect::<Vec<_>>();
    bricks.sort_by_key(|brick| height_of(brick));
    // value is height and the index of the brick that makes that height.
    let mut highest_by_position = HashMap::<Pos2, (usize, usize)>::new();
    let mut out: Vec<_> = (0..bricks.len()).map(|_| BrickInfo::default()).collect();
    for (i, brick) in bricks.into_iter().enumerate() {
        let mut max_height = 0;
        let mut resting_on = HashSet::<usize>::new();
        for &xyz in &brick {
            let xy = (xyz.0, xyz.1);
            if let Some(&(height, prev_index)) = highest_by_position.get(&xy) {
                if height > max_height {
                    max_height = height;
                    resting_on.clear();
                    resting_on.insert(prev_index);
                } else if height == max_height {
                    resting_on.insert(prev_index);
                }
            }
        }
        for brick_below in resting_on {
            out[brick_below].above.insert(i);
            out[i].below.insert(brick_below);
        }
        for &xyz in &brick {
            let xy = (xyz.0, xyz.1);
            let height_in_brick = xyz.2 - brick[0].2;
            highest_by_position.insert(xy, (max_height + 1 + height_in_brick, i));
        }
    }
    out
}

fn parse_brick(s: &str) -> Vec<Pos3> {
    let (start, end) = s.split_once('~').unwrap();
    let mut start_iter = start.split(',');
    let x1: usize = start_iter.next().unwrap().parse().unwrap();
    let y1: usize = start_iter.next().unwrap().parse().unwrap();
    let z1: usize = start_iter.next().unwrap().parse().unwrap();
    let mut end_iter = end.split(',');
    let x2: usize = end_iter.next().unwrap().parse().unwrap();
    let y2: usize = end_iter.next().unwrap().parse().unwrap();
    let z2: usize = end_iter.next().unwrap().parse().unwrap();
    let (min_x, max_x) = min_max(x1, x2);
    let (min_y, max_y) = min_max(y1, y2);
    let (min_z, max_z) = min_max(z1, z2);
    if min_x < max_x {
        (min_x..=max_x).map(|x| (x, min_y, min_z)).collect()
    } else if min_y < max_y {
        (min_y..=max_y).map(|y| (min_x, y, min_z)).collect()
    } else {
        (min_z..=max_z).map(|z| (min_x, min_y, z)).collect()
    }
}

fn min_max(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn height_of(brick: &[Pos3]) -> usize {
    brick[0].2
}
