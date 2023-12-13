use crate::{harness::input::RawInput, util::grid::Grid};

pub fn solve_part1(input: RawInput) -> usize {
    input
        .as_str()
        .split("\n\n")
        .map(|s| {
            let grid = Grid::parse_bytes(s);
            find_reflections(&grid)[0]
        })
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    input
        .as_str()
        .split("\n\n")
        .map(|s| {
            let grid = Grid::parse_bytes(s);
            let old = find_reflections(&grid)[0];
            for ij in grid.indices() {
                let mut grid = grid.clone();
                grid[ij] = if grid[ij] == b'#' { b'.' } else { b'#' };
                if let Some(&new) = find_reflections(&grid).iter().find(|&&x| x != old) {
                    return new;
                }
            }
            panic!("didn't find in part 2");
        })
        .sum()
}

fn find_reflections(grid: &Grid<u8>) -> Vec<usize> {
    let mut out = find_mirrored_rows(grid);
    out.iter_mut().for_each(|x| *x *= 100);
    out.extend(find_mirrored_rows(&grid.transpose()));
    out
}

fn find_mirrored_rows(grid: &Grid<u8>) -> Vec<usize> {
    let mut out = vec![];
    for i in 1..(grid.nrows()) {
        for ii in 1.. {
            if ii > i || i + ii - 1 >= grid.nrows() {
                out.push(i);
                break;
            }
            if !(0..grid.ncols()).all(|j| grid[[i - ii, j]] == grid[[i + ii - 1, j]]) {
                break;
            }
        }
    }
    out
}
