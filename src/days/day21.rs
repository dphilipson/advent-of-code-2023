use std::collections::{BTreeMap, HashMap};

use ndarray::Array2;

use crate::{
    harness::input::RawInput,
    util::{
        grid::Grid,
        search::{bfs, SeenState},
    },
};

pub fn solve_part1(input: RawInput) -> usize {
    let grid = Grid::parse_chars(input.as_str());
    let start_location = grid.indices().find(|&ij| grid[ij] == 'S').unwrap();
    let search_result = bfs::search(
        start_location,
        |&ij| grid.orthogonal_neighbors(ij).filter(|&ij| grid[ij] != '#'),
        |_| false,
    );
    search_result
        .seen_states
        .into_iter()
        .map(|seen| {
            let loc = seen.state;
            if (loc[0] == 0 || loc[0] == grid.ncols() - 1)
                && (loc[1] == 0 || loc[1] == grid.ncols() - 1)
            {
                // println!("{loc:?}, {}", seen.distance);
            }
            seen
        })
        .filter(|seen| seen.distance % 2 == 0 && seen.distance <= 64)
        .count()
}

pub fn solve_part2(input: RawInput) -> usize {
    let k = 202300;
    return 15221 * k * k + 15338 * k + 3882;
    // plan: find shortest distance to each inner spot from each outer corner.
    // Then iterate up and left
    // and up and right, etc.
    // then iterate straight up, and straight right. How to handle those?
    let grid = Grid::parse_chars(input.as_str());
    let mut hmms = vec![];

    // WANT: k = 202300
    for k in (0..9).step_by(2) {
        let sol = get_n_grids_solution(&grid, k);
        println!("k = {k}, sol = {sol}");
        hmms.push(sol);
    }
    let waas = (0..hmms.len() - 1)
        .map(|i| hmms[i + 1] - hmms[i])
        .collect::<Vec<_>>();
    for wah in &waas {
        println!("wah {wah}");
    }
    let thirds = (0..waas.len() - 1)
        .map(|i| waas[i + 1] - waas[i])
        .collect::<Vec<_>>();
    for third in &thirds {
        println!("third {third}");
    }
    todo!()

    // let start_location = grid.indices().find(|&ij| grid[ij] == 'S').unwrap();
    // let distance_counts_from_upper_left = get_counts_by_distances(&grid, [0, 0]);
    // let max_count_from_upper_left = distance_counts_from_upper_left.iter().filter(|&count| count > 0).last().unwrap();
    // let distance_counts_from_upper_right = get_counts_by_distances(&grid, [0, grid.ncols() - 1]);
    // let max_count_from_upper_right = distance_counts_from_upper_right.iter().filter(|&count| count > 0).last().unwrap();
    // let distance_counts_from_lower_left = get_counts_by_distances(&grid, [grid.nrows() - 1, 0]);
    // let distance_counts_from_lower_right =
    //     get_counts_by_distances(&grid, [grid.nrows() - 1, grid.ncols() - 1]);
    // let distance_counts_from_top_middle =
    //     get_counts_by_distances(&grid, [0, (grid.ncols() - 1) / 2]);
    // let distance_counts_from_left_middle =
    //     get_counts_by_distances(&grid, [(grid.nrows() - 1) / 2, 0]);
    // let distance_counts_from_right_middle =
    //     get_counts_by_distances(&grid, [(grid.nrows() - 1) / 2, grid.ncols() - 1]);
    // let distance_counts_from_bottom_middle =
    //     get_counts_by_distances(&grid, [grid.nrows() - 1, (grid.ncols() - 1) / 2]);
    // let num_steps = 26501365;
    // let edge_length = grid.ncols();
    // let corner_distance_from_start = edge_length - 1;
    // let grid_travel_bound = ((num_steps / edge_length) + 1) as isize;
    // let mut count = 0;
    // for i in (-grid_travel_bound)..=(grid_travel_bound) {
    //     for j in (-grid_travel_bound)..=(grid_travel_bound) {
    //         let grids_out = (i.abs() + j.abs()) as usize;
    //         if grids_out > grid_travel_bound {
    //             continue;
    //         }
    //         let remaining_steps = num_steps - corner_distance_from_start - 2 - edge_length * grids_out;
    //         if i > 0 && j > 0 {
    //             let aa = distance_counts_from_upper_left.get(remaining_steps).copied().
    //         }
    //     }
    // }

    // count

    // // let's get lower-right quadrant
    // // we need the "edge" region, where grid copies are partially filled.
    // // it must start after (26501365 - some_change) / 131 copies
    // // it must stop before (26501365 / 131) copies
    // // we can iterate over this range? what is that
    // let parity_spots_within_distance_from_upper_left: Vec<usize> = todo!();
    // let min_full_grid_distance: usize = todo!();
    // let max_full_grid_distance: usize = todo!();
    // // Need (i + j) in range [min_distance, max_distance).
    // let mut count = 0;
    // for sum in min_full_grid_distance..max_full_grid_distance {
    //     for i in 0..(sum + 1) {
    //         let j = sum - i;
    //         // parity is based on parity to travel to upper-right corner, plus parity of grid travels.
    //         // TODO: if this is negative, continue. Or do we need that?
    //         let remaining_steps =
    //             num_steps - start_distance_from_lower_right - 2 - sum * edge_length;
    //         count += parity_spots_within_distance_from_upper_left[remaining_steps];
    //     }
    // }
    // // Do this for each quadrant, then what?
    // todo!()

    // let mut distances_from_top_left: Grid<usize> =
    //     Grid(Array2::default((grid.nrows(), grid.ncols())));
    // let mut distances_from_top_right: Grid<usize> =
    //     Grid(Array2::default((grid.nrows(), grid.ncols())));
    // let mut distances_from_bottom_left: Grid<usize> =
    //     Grid(Array2::default((grid.nrows(), grid.ncols())));
    // let mut distances_from_bottom_right: Grid<usize> =
    //     Grid(Array2::default((grid.nrows(), grid.ncols())));

    // let counts_by_distances_from_top_left = top_left_result.seen_states.into_iter();
}

fn get_n_grids_solution(grid: &Grid<char>, k: usize) -> usize {
    let n = 2 * k + 1;
    let mut bigger_grid = Grid(Array2::default((grid.nrows() * n, grid.ncols() * n)));
    for ii in 0..n {
        for jj in 0..n {
            for i in 0..grid.nrows() {
                for j in 0..grid.ncols() {
                    bigger_grid[[ii * grid.nrows() + i, jj * grid.ncols() + j]] = grid[[i, j]];
                }
            }
        }
    }
    let grid = bigger_grid;
    let max_distance = 65 + 131 * k;
    let start_location = [(grid.nrows()) / 2, (grid.ncols()) / 2];
    let search_result = bfs::search(
        start_location,
        |&ij| grid.orthogonal_neighbors(ij).filter(|&ij| grid[ij] != '#'),
        |_| false,
    );
    search_result
        .seen_states
        .into_iter()
        .filter(|seen| seen.distance % 2 == 1 && seen.distance <= max_distance)
        .count()
}

fn get_counts_by_distances(grid: &Grid<char>, start: [usize; 2]) -> Vec<usize> {
    let mut out = vec![0; 260];
    let search_result = bfs::search(
        start,
        |&ij| grid.orthogonal_neighbors(ij).filter(|&ij| grid[ij] != '#'),
        |_| false,
    );
    search_result.seen_states.into_iter().for_each(|seen| {
        let mut n = seen.distance;
        while n > 1 {
            out[n] += 1;
            n -= 2;
        }
    });
    out
}
