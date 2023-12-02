use std::str::FromStr;

use regex::Regex;

use crate::{harness::input::RawInput, regex, util::re};

pub fn solve_part1(input: RawInput) -> u32 {
    input
        .per_line(|line| parse_game(line.as_str()))
        .filter(|game| game.max_red <= 12 && game.max_green <= 13 && game.max_blue <= 14)
        .map(|game| game.id)
        .sum()
}

pub fn solve_part2(input: RawInput) -> u32 {
    input
        .per_line(|line| {
            let game = parse_game(line.as_str());
            game.max_red * game.max_green * game.max_blue
        })
        .sum()
}

#[derive(Debug)]
struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

fn parse_game(s: &str) -> Game {
    let (id,) = re::parse_with_regex::<(u32,)>(regex!(r"Game (\d+)"), s).unwrap();
    Game {
        id,
        max_red: parse_max_of_color(regex!(r"(\d+) red"), s),
        max_green: parse_max_of_color(regex!(r"(\d+) green"), s),
        max_blue: parse_max_of_color(regex!(r"(\d+) blue"), s),
    }
}

fn parse_max_of_color(color_re: &Regex, s: &str) -> u32 {
    color_re
        .captures_iter(s)
        .map(|captures| {
            let (_, [count]) = captures.extract();
            u32::from_str(count).unwrap()
        })
        .max()
        .unwrap_or_default()
}
