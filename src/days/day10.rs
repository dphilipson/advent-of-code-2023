use std::collections::HashSet;

use crate::{
    harness::input::RawInput,
    util::{
        grid::Grid,
        search::{bfs, SeenState},
    },
};

pub fn solve_part1(input: RawInput) -> usize {
    let grid = Grid::parse_bytes(input.as_str());
    let start_location = get_start_location(&grid);
    let start_pipe = get_start_pipe(&grid, start_location);
    get_path(&grid, start_location, start_pipe)
        .into_iter()
        .map(|seen| seen.distance)
        .max()
        .unwrap()
}

pub fn solve_part2(input: RawInput) -> usize {
    let grid = Grid::parse_bytes(input.as_str());
    let start_location = get_start_location(&grid);
    let start_pipe = get_start_pipe(&grid, start_location);
    let pipes = get_path(&grid, start_location, start_pipe)
        .into_iter()
        .map(|seen| seen.state)
        .collect::<HashSet<_>>();
    let mut count = 0;
    for i in 0..grid.nrows() {
        let mut is_inside = false;
        for j in 0..grid.ncols() {
            if pipes.contains(&[i, j]) {
                if b"|JL".contains(&get_pipe_at(&grid, [i, j], start_pipe)) {
                    is_inside = !is_inside;
                }
            } else if is_inside {
                count += 1;
            }
        }
    }
    count
}

fn get_start_location(grid: &Grid<u8>) -> [usize; 2] {
    grid.indices().find(|&ij| grid[ij] == b'S').unwrap()
}

fn get_start_pipe(grid: &Grid<u8>, start_location: [usize; 2]) -> u8 {
    let [i, j] = start_location;
    let connects_top = i > 0 && b"|7F".contains(&grid[[i - 1, j]]);
    let connects_bottom = i < grid.nrows() - 1 && b"|JL".contains(&grid[[i + 1, j]]);
    let connects_left = j > 0 && b"-7J".contains(&grid[[i, j - 1]]);
    match (connects_top, connects_bottom, connects_left) {
        (true, true, false) => b'|',
        (true, false, true) => b'J',
        (true, false, false) => b'L',
        (false, true, true) => b'7',
        (false, true, false) => b'F',
        (false, false, true) => b'-',
        _ => panic!("invalid start"),
    }
}

fn get_path(
    grid: &Grid<u8>,
    start_location: [usize; 2],
    start_pipe: u8,
) -> Vec<SeenState<[usize; 2]>> {
    bfs::search(
        start_location,
        |&[i, j]| match get_pipe_at(grid, [i, j], start_pipe) {
            b'|' => vec![[i - 1, j], [i + 1, j]],
            b'-' => vec![[i, j - 1], [i, j + 1]],
            b'L' => vec![[i - 1, j], [i, j + 1]],
            b'J' => vec![[i - 1, j], [i, j - 1]],
            b'7' => vec![[i + 1, j], [i, j - 1]],
            b'F' => vec![[i + 1, j], [i, j + 1]],
            _ => panic!("oh no"),
        },
        |_| false,
    )
    .seen_states
}

fn get_pipe_at(grid: &Grid<u8>, location: [usize; 2], start_pipe: u8) -> u8 {
    let pipe = grid[location];
    if pipe == b'S' {
        start_pipe
    } else {
        pipe
    }
}
