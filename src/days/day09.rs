use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> f64 {
    input
        .per_line(|line| {
            let nums = line.split_whitespace::<f64>();
            interpolate(&nums, nums.len() as f64)
        })
        .sum()
}

pub fn solve_part2(input: RawInput) -> f64 {
    input
        .per_line(|line| interpolate(&line.split_whitespace::<f64>(), -1.0))
        .sum()
}

fn interpolate(ys: &[f64], new_x: f64) -> f64 {
    let n = ys.len();
    let result: f64 = ys
        .iter()
        .enumerate()
        .map(|(x, &y)| {
            let x = x as f64;
            let coefficient: f64 = (0..n)
                .map(|i| i as f64)
                .filter(|&i| i != x)
                .map(|i| (new_x - i) / (x - i))
                .product();
            coefficient * y
        })
        .sum();
    result.round() as f64
}
