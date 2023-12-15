use std::{array, str::FromStr};

use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> usize {
    input.as_str().split(',').into_iter().map(hash).sum()
}

pub fn solve_part2(input: RawInput) -> usize {
    let mut boxes: [Vec<(&str, usize)>; 256] = array::from_fn(|_| vec![]);
    input.as_str().split(",").for_each(|s| {
        if s.contains('=') {
            let (label, lens) = s.split_once('=').unwrap();
            let boxx = &mut boxes[hash(label)];
            let lens = usize::from_str(lens).unwrap();
            if let Some((_, lens_in_box)) = boxx.iter_mut().find(|(l, _)| *l == label) {
                *lens_in_box = lens;
            } else {
                boxx.push((label, lens));
            }
        } else {
            let label = &s[..(s.len() - 1)];
            let boxx = &mut boxes[hash(label)];
            if let Some(existing_i) = boxx.iter().position(|(l, _)| *l == label) {
                boxx.remove(existing_i);
            }
        }
    });
    boxes
        .into_iter()
        .enumerate()
        .map(|(i, boxx)| {
            (i + 1)
                * boxx
                    .into_iter()
                    .enumerate()
                    .map(|(j, (_, lens))| (j + 1) * lens)
                    .sum::<usize>()
        })
        .sum()
}

fn hash(s: &str) -> usize {
    s.as_bytes()
        .into_iter()
        .fold(0, |acc, &b| (17 * (acc + (b as usize))) % 256)
}
