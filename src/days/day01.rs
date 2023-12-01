use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> u32 {
    input
        .per_line(|line| {
            let chars = line.chars();
            let first = chars.iter().filter_map(|c| c.to_digit(10)).next().unwrap();
            let last = chars.iter().filter_map(|c| c.to_digit(10)).last().unwrap();
            10 * first + last
        })
        .sum()
}

pub fn solve_part2(input: RawInput) -> u32 {
    let numbers = &[
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ];

    input
        .per_line(|line| {
            let first = numbers
                .iter()
                .enumerate()
                .filter_map(|(i, s)| line.as_str().find(s).map(|pos| (i, pos)))
                .min_by_key(|(_, pos)| *pos)
                .unwrap()
                .0 as u32
                % 10;
            let last = numbers
                .iter()
                .enumerate()
                .filter_map(|(i, s)| line.as_str().rfind(s).map(|pos| (i, pos)))
                .max_by_key(|(_, pos)| *pos)
                .unwrap()
                .0 as u32
                % 10;
            10 * first + last
        })
        .sum()
}
