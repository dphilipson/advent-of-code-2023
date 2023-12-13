use crate::{harness::input::RawInput, util::grid::Grid};

pub fn solve_part1(input: RawInput) -> usize {
    input
        .as_str()
        .split("\n\n")
        .map(|s| {
            let grid = Grid::parse_bytes(s);
            compute(&grid)[0]
        })
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    input
        .as_str()
        .split("\n\n")
        .map(|s| {
            let grid = Grid::parse_bytes(s);
            let old = compute(&grid)[0];
            for ij in grid.indices() {
                let mut grid = grid.clone();
                grid[ij] = if grid[ij] == b'#' { b'.' } else { b'#' };
                if let Some(&new) = compute(&grid).iter().find(|&&x| x != old) {
                    return new;
                }
            }
            panic!("didn't find in part 2");
        })
        .sum()
}

fn compute(grid: &Grid<u8>) -> Vec<usize> {
    let mut out = vec![];
    for i in 1..(grid.nrows()) {
        for ii in 1.. {
            if ii > i || i + ii - 1 >= grid.nrows() {
                out.push(100 * i);
                break;
            }
            if !(0..grid.ncols()).all(|j| grid[[i - ii, j]] == grid[[i + ii - 1, j]]) {
                break;
            }
        }
    }
    for j in 1..(grid.ncols()) {
        for jj in 1.. {
            if jj > j || j + jj - 1 >= grid.ncols() {
                out.push(j);
                break;
            }
            if !(0..grid.nrows()).all(|i| grid[[i, j - jj]] == grid[[i, j + jj - 1]]) {
                break;
            }
        }
    }
    out
}
