use std::collections::{BTreeMap, HashMap};

use ndarray::Array2;

use crate::{
    harness::input::RawInput,
    regex,
    util::{grid::Grid, re::parse_with_regex, search::bfs},
};

pub fn solve_part1(input: RawInput) -> usize {
    todo!();
    let mut grid = Grid(Array2::default((1000, 1000)));
    let mut pos = [500, 500];
    grid[pos] = true;
    for line in input.as_str().lines() {
        let (dir, distance) =
            parse_with_regex::<(char, usize)>(regex!(r"(.) (\d+) .*"), line).unwrap();
        match dir {
            'R' => {
                for _ in 0..distance {
                    pos[1] += 1;
                    grid[pos] = true;
                }
            }
            'U' => {
                for _ in 0..distance {
                    pos[0] -= 1;
                    grid[pos] = true;
                }
            }
            'L' => {
                for _ in 0..distance {
                    pos[1] -= 1;
                    grid[pos] = true;
                }
            }
            'D' => {
                for _ in 0..distance {
                    pos[0] += 1;
                    grid[pos] = true;
                }
            }
            _ => panic!("what"),
        }
    }
    let fill_search = bfs::search(
        [501, 501],
        |&ij| grid.orthogonal_neighbors(ij).filter(|&ij| !grid[ij]),
        |_| false,
    );
    fill_search.seen_states.len() + grid.indices().filter(|&ij| grid[ij]).count()
    // let mut count = 0;
    // for i in 0..grid.nrows() {
    //     let mut is_inside = false;
    //     let mut prev_was_wall = false;
    //     for j in 0..grid.ncols() {
    //         if grid[[i, j]] {
    //             is_inside = !is_inside;
    //         }
    //         if grid[[i, j]] || is_inside {
    //             count += 1;
    //         }
    //     }
    // }
    // println!("{:#?}", grid.map(|&b| if b { '#' } else { '.' }));
    // count
}

pub fn solve_part2(input: RawInput) -> usize {
    // Map of x -> y -> bool, sorted ys
    let mut filled: HashMap<i64, BTreeMap<i64, bool>> = HashMap::new();
    let mut min_i = 0;
    let mut max_i = 0;
    let mut pos = [0_i64, 0];
    let mut wah = 0;
    for line in input.as_str().lines() {
        let (hex_part) = parse_with_regex::<(String,)>(regex!(r".*#(.*)\)"), line).unwrap();
        let distance = i64::from_str_radix(&hex_part.0[0..5], 16).unwrap();
        wah += distance;
        match hex_part.0.as_bytes()[5] {
            b'0' => {
                for _ in 0..distance {
                    pos[1] += 1;
                    filled.entry(pos[0]).or_default().insert(pos[1], true);
                }
            }
            b'3' => {
                for _ in 0..distance {
                    pos[0] -= 1;
                    filled.entry(pos[0]).or_default().insert(pos[1], true);
                }
            }
            b'2' => {
                for _ in 0..distance {
                    pos[1] -= 1;
                    filled.entry(pos[0]).or_default().insert(pos[1], true);
                }
            }
            b'1' => {
                for _ in 0..distance {
                    pos[0] += 1;
                    filled.entry(pos[0]).or_default().insert(pos[1], true);
                }
            }
            _ => panic!("what"),
        }
        min_i = min_i.min(pos[0]);
        max_i = max_i.max(pos[0]);
    }
    println!("{wah}");
    let mut count = 0;
    for i in min_i..=max_i {
        let filled2 = filled.get(&i).unwrap();
        let mut is_inside = false;
        let mut last_j: Option<i64> = None;
        for &j in filled2.keys() {
            count += 1;
            if let Some(last_j) = last_j {
                if is_inside {
                    count += j - last_j - 1;
                }
            }
            // if the space jsut above it is filled, then crosses
            if let Some(blah) = filled.get(&(i - 1)) {
                if blah.contains_key(&j) {
                    is_inside = !is_inside;
                }
            }
            last_j = Some(j);
        }
    }
    count as usize
}
