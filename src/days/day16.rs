use std::collections::HashSet;

use crate::{
    harness::input::RawInput,
    util::{grid::Grid, search::bfs},
};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

type State = ([usize; 2], Direction);

use Direction::*;

pub fn solve_part1(input: RawInput) -> usize {
    let grid = Grid::parse_bytes(input.as_str());
    solve(&grid, ([0, 0], Right))
}

pub fn solve_part2(input: RawInput) -> usize {
    let grid = Grid::parse_bytes(input.as_str());
    let mut initial_states = vec![];
    for i in 0..grid.nrows() {
        initial_states.push(([i, 0], Right));
        initial_states.push(([i, grid.ncols() - 1], Left));
    }
    for j in 0..grid.ncols() {
        initial_states.push(([0, j], Down));
        initial_states.push(([grid.nrows() - 1, j], Up));
    }
    initial_states
        .into_iter()
        .map(|initial_state| solve(&grid, initial_state))
        .max()
        .unwrap()
}

fn solve(grid: &Grid<u8>, initial_state: State) -> usize {
    let search_result = bfs::search(
        initial_state,
        |&(ij, dir)| match grid[ij] {
            b'.' => match dir {
                Right => right(grid, ij),
                Up => up(grid, ij),
                Left => left(grid, ij),
                Down => down(grid, ij),
            },
            b'|' => match dir {
                Left | Right => {
                    let mut outs = vec![];
                    outs.extend(up(grid, ij));
                    outs.extend(down(grid, ij));
                    outs
                }
                Up => up(grid, ij),
                Down => down(grid, ij),
            },
            b'-' => match dir {
                Up | Down => {
                    let mut outs = vec![];
                    outs.extend(right(grid, ij));
                    outs.extend(left(grid, ij));
                    outs
                }
                Right => right(grid, ij),
                Left => left(grid, ij),
            },
            b'/' => match dir {
                Right => up(grid, ij),
                Up => right(grid, ij),
                Left => down(grid, ij),
                Down => left(grid, ij),
            },
            b'\\' => match dir {
                Right => down(grid, ij),
                Up => left(grid, ij),
                Left => up(grid, ij),
                Down => right(grid, ij),
            },
            _ => panic!("Bad character"),
        },
        |_| false,
    );
    search_result
        .seen_states
        .iter()
        .map(|seen| seen.state.0)
        .collect::<HashSet<_>>()
        .len()
}

fn right(grid: &Grid<u8>, [i, j]: [usize; 2]) -> Vec<State> {
    if j < grid.ncols() - 1 {
        vec![([i, j + 1], Right)]
    } else {
        vec![]
    }
}

fn up(_: &Grid<u8>, [i, j]: [usize; 2]) -> Vec<State> {
    if i > 0 {
        vec![([i - 1, j], Up)]
    } else {
        vec![]
    }
}

fn left(_: &Grid<u8>, [i, j]: [usize; 2]) -> Vec<State> {
    if j > 0 {
        vec![([i, j - 1], Left)]
    } else {
        vec![]
    }
}

fn down(grid: &Grid<u8>, [i, j]: [usize; 2]) -> Vec<State> {
    if i < grid.nrows() - 1 {
        vec![([i + 1, j], Down)]
    } else {
        vec![]
    }
}
