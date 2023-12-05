use std::str::FromStr;

use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> u64 {
    let input = input.as_str();
    let seeds = get_seeds(&input);
    let mappings = get_mappings(&input);
    seeds
        .iter()
        .map(|&seed| apply_mappingses(&mappings, seed))
        .min()
        .unwrap()
}

pub fn solve_part2(input: RawInput) -> u64 {
    let input = input.as_str();
    let ranges = get_ranges(&input);
    let mappings = get_mappings(&input);
    ranges
        .iter()
        .flat_map(|&seed| apply_mappingses_to_range(&mappings, seed))
        .map(|range| range.start)
        .min()
        .unwrap()
}

fn get_seeds(s: &str) -> Vec<u64> {
    s.lines()
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| u64::from_str(s).unwrap())
        .collect()
}

fn get_ranges(s: &str) -> Vec<Range> {
    let seeds = get_seeds(s);
    (0..seeds.len())
        .step_by(2)
        .map(|i| Range {
            start: seeds[i],
            len: seeds[i + 1],
        })
        .collect()
}

#[derive(Clone, Copy, Debug)]
struct Mapping {
    dest_start: u64,
    src_start: u64,
    len: u64,
}

fn get_mappings(s: &str) -> Vec<Vec<Mapping>> {
    s.split("\n\n")
        .skip(1)
        .map(|group| {
            group
                .lines()
                .skip(1)
                .map(|line| {
                    let mut iter = line
                        .split_ascii_whitespace()
                        .map(|s| u64::from_str(s).unwrap());
                    Mapping {
                        dest_start: iter.next().unwrap(),
                        src_start: iter.next().unwrap(),
                        len: iter.next().unwrap(),
                    }
                })
                .collect()
        })
        .collect()
}

fn apply_mapping(mapping: &Mapping, n: u64) -> Option<u64> {
    if (mapping.src_start..(mapping.src_start + mapping.len)).contains(&n) {
        Some(mapping.dest_start + n - mapping.src_start)
    } else {
        None
    }
}

fn apply_mappings(mappings: &[Mapping], n: u64) -> u64 {
    for mapping in mappings {
        if let Some(out) = apply_mapping(mapping, n) {
            return out;
        }
    }
    return n;
}

fn apply_mappingses(mappingses: &[Vec<Mapping>], mut n: u64) -> u64 {
    for mappings in mappingses {
        n = apply_mappings(mappings, n);
    }
    n
}

#[derive(Clone, Copy, Debug)]
struct Range {
    start: u64,
    len: u64,
}

// Returns optionally: a chunk to remove from the start range, and a range to
// include in the next round of mappings.
fn apply_mapping_to_range(mapping: &Mapping, range: Range) -> Option<(Range, Range)> {
    let src_start = range.start.max(mapping.src_start);
    let src_end = (range.start + range.len).min(mapping.src_start + mapping.len);
    if src_start < src_end {
        let len = src_end - src_start;
        Some((
            Range {
                start: src_start,
                len,
            },
            Range {
                start: mapping.dest_start + src_start - mapping.src_start,
                len,
            },
        ))
    } else {
        None
    }
}

fn apply_mappings_to_range(mappings: &[Mapping], range: Range) -> Vec<Range> {
    let mut to_cut_out = vec![];
    let mut to_include_next = vec![];
    for mapping in mappings {
        if let Some((cut_out, include_next)) = apply_mapping_to_range(mapping, range) {
            to_cut_out.push(cut_out);
            to_include_next.push(include_next);
        }
    }
    if to_cut_out.is_empty() {
        return vec![range];
    }
    to_cut_out.sort_by_key(|range| range.start);
    let mut previous_end = range.start;
    for cut_out in to_cut_out {
        let len = cut_out.start - previous_end;
        if len > 0 {
            to_include_next.push(Range {
                start: previous_end,
                len,
            });
        }
        previous_end = cut_out.start + cut_out.len;
    }
    to_include_next
}

fn apply_mappingses_to_range(mappingses: &[Vec<Mapping>], range: Range) -> Vec<Range> {
    let mut ranges = vec![range];
    for mappings in mappingses {
        let mut next_ranges = vec![];
        for range in ranges {
            next_ranges.extend(apply_mappings_to_range(mappings, range));
        }
        ranges = next_ranges;
    }
    ranges
}
