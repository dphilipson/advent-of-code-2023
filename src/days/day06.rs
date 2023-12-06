use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> u64 {
    let mut lines = input.as_str().lines();
    let times = parse_line(lines.next().unwrap());
    let distances = parse_line(lines.next().unwrap());
    (0..times.len())
        .map(|i| get_winning_time_count(times[i], distances[i]))
        .product()
}

pub fn solve_part2(input: RawInput) -> u64 {
    let mut lines = input.as_str().lines();
    let time = parse_mashed_line(lines.next().unwrap());
    let distance = parse_mashed_line(lines.next().unwrap());
    get_winning_time_count(time, distance)
}

fn parse_line(line: &str) -> Vec<u64> {
    line.split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_mashed_line(line: &str) -> u64 {
    line.split_once(':')
        .unwrap()
        .1
        .replace(' ', "")
        .parse()
        .unwrap()
}

fn get_winning_time_count(race_time: u64, record: u64) -> u64 {
    let race_time = race_time as f64;
    let record = record as f64;
    let root = (race_time * race_time - 4.0 * (record + 1.0)).sqrt();
    let low_bound = ((race_time - root) / 2.0).ceil() as u64;
    let high_bound = ((race_time + root) / 2.0).floor() as u64;
    high_bound - low_bound + 1
}
