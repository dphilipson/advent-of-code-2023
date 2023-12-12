use std::{collections::HashMap, str::FromStr};

use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    input
        .per_line(|line| {
            let (spots, counts) = parse_spots_and_counts(line.as_str());
            solve(spots, &counts)
        })
        .sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    input
        .per_line(|line| {
            let (spots, counts) = parse_spots_and_counts(line.as_str());
            let mut new_spots = Vec::new();
            let mut new_counts = Vec::new();
            for _ in 0..5 {
                new_spots.extend(spots.to_owned());
                new_spots.push(b'?');
                new_counts.extend(counts.clone());
            }
            new_spots.pop();
            solve(&new_spots, &new_counts)
        })
        .sum()
}

fn solve(spots: &[u8], counts: &[usize]) -> usize {
    solve_recursive(spots, counts, 0, 0, &mut HashMap::new())
}

fn solve_recursive(
    spots: &[u8],
    counts: &[usize],
    i: usize,
    j: usize,
    memo_map: &mut HashMap<[usize; 2], usize>,
) -> usize {
    if let Some(&cached) = memo_map.get(&[i, j]) {
        return cached;
    }
    let mut f = |i: usize, j: usize| solve_recursive(spots, counts, i, j, memo_map);
    let out = (|| {
        if i >= spots.len() {
            return if j == counts.len() { 1 } else { 0 };
        }
        if j == counts.len() {
            return if i > spots.len() || spots[i..].iter().all(|&spot| spot != b'#') {
                1
            } else {
                0
            };
        }
        if spots[i] == b'.' {
            return f(i + 1, j);
        }
        let i_end = i + counts[j];
        let can_place_here = i_end <= spots.len()
            && spots[i..i_end].iter().all(|&spot| spot != b'.')
            && (i_end == spots.len() || spots[i_end] != b'#');
        if !can_place_here && spots[i] == b'#' {
            return 0;
        }
        let skip_counts = if spots[i] == b'?' { f(i + 1, j) } else { 0 };
        let take_counts = if can_place_here {
            f(i_end + 1, j + 1)
        } else {
            0
        };
        skip_counts + take_counts
    })();
    memo_map.insert([i, j], out);
    out
}

fn parse_spots_and_counts<'a>(line: &'a str) -> (&'a [u8], Vec<usize>) {
    let (spots, counts) = line.split_once(' ').unwrap();
    let spots = spots.as_bytes();
    let counts = counts
        .split(',')
        .map(|s| usize::from_str(s).unwrap())
        .collect();
    (spots, counts)
}
