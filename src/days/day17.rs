use crate::{
    harness::input::RawInput,
    util::{grid::Grid, search::dijkstra},
};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

use Direction::*;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
struct State {
    location: [usize; 2],
    steps_in_direction: usize,
    direction: Direction,
}

impl State {
    fn right(&self) -> Self {
        let [i, j] = self.location;
        let steps_in_direction = if self.direction == Right {
            self.steps_in_direction + 1
        } else {
            1
        };
        Self {
            location: [i, j + 1],
            steps_in_direction,
            direction: Right,
        }
    }

    fn up(&self) -> Self {
        let [i, j] = self.location;
        let steps_in_direction = if self.direction == Up {
            self.steps_in_direction + 1
        } else {
            1
        };
        Self {
            location: [i.wrapping_sub(1), j],
            steps_in_direction,
            direction: Up,
        }
    }

    fn left(&self) -> Self {
        let [i, j] = self.location;
        let steps_in_direction = if self.direction == Left {
            self.steps_in_direction + 1
        } else {
            1
        };
        Self {
            location: [i, j.wrapping_sub(1)],
            steps_in_direction,
            direction: Left,
        }
    }

    fn down(&self) -> Self {
        let [i, j] = self.location;
        let steps_in_direction = if self.direction == Down {
            self.steps_in_direction + 1
        } else {
            1
        };
        Self {
            location: [i + 1, j],
            steps_in_direction,
            direction: Down,
        }
    }
}

pub fn solve_part1(input: RawInput) -> usize {
    solve(input, 0, 3)
}

pub fn solve_part2(input: RawInput) -> usize {
    solve(input, 4, 10)
}

pub fn solve(input: RawInput, min_travel_distance: usize, max_travel_distance: usize) -> usize {
    let grid = Grid::parse_digits(input.as_str()).map(|&n| n as usize);
    [Right, Down]
        .into_iter()
        .map(|initial_direction| {
            let search_result = dijkstra::search(
                State {
                    location: [0, 0],
                    steps_in_direction: 0,
                    direction: initial_direction,
                },
                |&state| {
                    let unfiltered_nexts = match state.direction {
                        Right => [state.right(), state.up(), state.down()],
                        Up => [state.up(), state.left(), state.right()],
                        Left => [state.left(), state.down(), state.up()],
                        Down => [state.down(), state.right(), state.left()],
                    };
                    unfiltered_nexts
                        .into_iter()
                        .filter(|new_state| {
                            let [i, j] = new_state.location;
                            (state.steps_in_direction >= min_travel_distance
                                || state.direction == new_state.direction)
                                && new_state.steps_in_direction <= max_travel_distance
                                && (0..grid.nrows()).contains(&i)
                                && (0..grid.ncols()).contains(&j)
                        })
                        .map(|state| (state, grid[state.location] as usize))
                        .collect::<Vec<_>>()
                },
                |state| state.location == [grid.nrows() - 1, grid.ncols() - 1],
            );
            search_result.goal_state().unwrap().distance
        })
        .min()
        .unwrap()
}
