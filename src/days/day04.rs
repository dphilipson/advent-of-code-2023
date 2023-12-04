use std::{collections::HashSet, str::FromStr};

use crate::{harness::input::RawInput, regex, util::re};

pub fn solve_part1(input: RawInput) -> u32 {
    input
        .per_line(|line| {
            let winning_count = get_winning_count(line.as_str());
            if winning_count == 0 {
                0
            } else {
                1 << (winning_count - 1)
            }
        })
        .sum()
}

pub fn solve_part2(input: RawInput) -> u32 {
    let input = input.as_str();
    let mut copy_count = vec![1; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let winning_count = get_winning_count(line);
        for j in (i + 1)..((i + 1 + winning_count).min(copy_count.len())) {
            copy_count[j] += copy_count[i];
        }
    }
    copy_count.iter().sum()
}

fn get_winning_count(line: &str) -> usize {
    let re = regex!(r"(?:.+:)(.+)\|(.+)");
    let (winning, on_card) = re::parse_with_regex::<(String, String)>(re, line).unwrap();
    let winning_set = winning
        .split_ascii_whitespace()
        .map(|s| u32::from_str(s).unwrap())
        .collect::<HashSet<_>>();
    on_card
        .split_ascii_whitespace()
        .map(|s| u32::from_str(s).unwrap())
        .filter(|n| winning_set.contains(n))
        .count()
}
