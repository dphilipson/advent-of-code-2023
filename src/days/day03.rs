use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::{harness::input::RawInput, regex, util::grid::Grid};

pub fn solve_part1(input: RawInput) -> u32 {
    let grid = Grid::parse_chars(input.as_str());
    let lines = input.as_str().lines().collect::<Vec<_>>();
    let mut sum = 0;
    for i in 0..grid.nrows() {
        for m in regex!(r"(\d+)").find_iter(lines[i]) {
            for j in m.start()..m.end() {
                if grid.neighbors([i, j]).any(|ij| is_gear(grid[ij])) {
                    sum += u32::from_str(m.as_str()).unwrap();
                    break;
                }
            }
        }
    }
    sum
}

pub fn solve_part2(input: RawInput) -> u32 {
    let grid = Grid::parse_chars(input.as_str());
    let lines = input.as_str().lines().collect::<Vec<_>>();
    let mut gear_to_neighbors = HashMap::<[usize; 2], Vec<u32>>::new();
    let mut handled = HashSet::<([usize; 2], [usize; 2])>::new();
    for i in 0..grid.nrows() {
        for m in regex!(r"(\d+)").find_iter(lines[i]) {
            for j in m.start()..m.end() {
                for ij in grid.neighbors([i, j]) {
                    if is_gear(grid[ij]) {
                        let handle = (ij, [i, m.start()]);
                        if !handled.contains(&handle) {
                            handled.insert(handle);
                            let n = u32::from_str(m.as_str()).unwrap();
                            gear_to_neighbors.entry(ij).or_default().push(n);
                        }
                    }
                }
            }
        }
    }
    gear_to_neighbors
        .values()
        .filter(|ns| ns.len() == 2)
        .map(|ns| ns[0] * ns[1])
        .sum()
}

fn is_gear(c: char) -> bool {
    c != '.' && !c.is_digit(10)
}
