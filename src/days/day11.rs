use std::collections::BTreeSet;

use crate::{harness::input::RawInput, util::grid::Grid};

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, 2)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, 1_000_000)
}

pub fn solve(input: RawInput, expansion: usize) -> usize {
    let grid = Grid::parse_bytes(input.as_str());
    let empty_rows = (0..grid.nrows())
        .filter(|&i| (0..grid.ncols()).all(|j| grid[[i, j]] == b'.'))
        .collect::<BTreeSet<_>>();
    let empty_cols = (0..grid.ncols())
        .filter(|&j| (0..grid.nrows()).all(|i| grid[[i, j]] == b'.'))
        .collect::<BTreeSet<_>>();
    let galaxy_locations = grid
        .indices()
        .filter(|&ij| grid[ij] == b'#')
        .collect::<Vec<_>>();
    let mut sum = 0;
    for a in 0..galaxy_locations.len() {
        for b in (a + 1)..galaxy_locations.len() {
            let [i1, j1] = galaxy_locations[a];
            let [i2, j2] = galaxy_locations[b];
            let min_i = i1.min(i2);
            let max_i = i1.max(i2);
            let empty_row_count = empty_rows.range(min_i..max_i).count();
            let min_j = j1.min(j2);
            let max_j = j1.max(j2);
            let empty_col_count = empty_cols.range(min_j..max_j).count();
            sum += max_i - min_i + max_j - min_j
                + (expansion - 1) * (empty_row_count + empty_col_count);
        }
    }
    sum
}
