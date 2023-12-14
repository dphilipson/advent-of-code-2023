use std::collections::HashMap;

use crate::{harness::input::RawInput, util::grid::Grid};

pub fn solve_part1(input: RawInput) -> usize {
    let mut grid = Grid::parse_bytes(input.as_str());
    slide_up(&mut grid);
    get_weight(&grid)
}

pub fn solve_part2(input: RawInput) -> usize {
    let mut seen_grids = HashMap::<Vec<u8>, usize>::new();
    let mut grid = Grid::parse_bytes(input.as_str());
    let mut target_step: Option<usize> = None;
    for step in 0.. {
        for _ in 0..4 {
            slide_up(&mut grid);
            rotate_right(&mut grid);
        }
        let slice = grid.0.as_slice().unwrap().to_owned();
        if target_step.is_none() {
            if let Some(&last_seen) = seen_grids.get(&slice) {
                let cycle_length = step - last_seen;
                let additional_steps = (1_000_000_000 - step) % cycle_length;
                target_step = Some(step + additional_steps);
            } else {
                seen_grids.insert(slice, step);
            }
        }
        if target_step == Some(step + 1) {
            return get_weight(&grid);
        }
    }
    unreachable!()
}

fn slide_up(grid: &mut Grid<u8>) {
    for row in 1..grid.nrows() {
        for col in 0..grid.ncols() {
            if grid[[row, col]] == b'O' {
                let target = (0..row)
                    .rev()
                    .take_while(|&back_row| grid[[back_row, col]] == b'.')
                    .last();
                if let Some(target) = target {
                    grid[[row, col]] = b'.';
                    grid[[target, col]] = b'O';
                }
            }
        }
    }
}

fn get_weight(grid: &Grid<u8>) -> usize {
    let mut sum = 0;
    for [row, col] in grid.indices() {
        if grid[[row, col]] == b'O' {
            sum += grid.nrows() - row;
        }
    }
    return sum;
}

fn rotate_right(grid: &mut Grid<u8>) {
    let n = grid.ncols();
    for i in 0..(n / 2) {
        for j in 0..(n / 2) {
            let temp = grid[[i, j]];
            grid[[i, j]] = grid[[n - 1 - j, i]];
            grid[[n - 1 - j, i]] = grid[[n - 1 - i, n - 1 - j]];
            grid[[n - 1 - i, n - 1 - j]] = grid[[j, n - 1 - i]];
            grid[[j, n - 1 - i]] = temp;
        }
    }
}
