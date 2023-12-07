use std::{cmp::Ordering, str::FromStr};

use crate::harness::input::RawInput;

pub fn solve_part1(input: RawInput) -> u32 {
    solve(input, b"23456789TJQKA", false)
}

pub fn solve_part2(input: RawInput) -> u32 {
    solve(input, b"J23456789TQKA", true)
}

fn solve(input: RawInput, ranks: &'static [u8; 13], use_jokers: bool) -> u32 {
    let mut hands_and_bids = input
        .per_line(|line| {
            let (hand, bid) = line.as_str().split_once(' ').unwrap();
            let hand = hand
                .bytes()
                .map(|b| ranks.iter().position(|&b2| b2 == b).unwrap())
                .collect::<Vec<_>>();
            let bid = u32::from_str(bid).unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();
    hands_and_bids.sort_by(|a, b| cmp_hands(&a.0, &b.0, use_jokers));
    hands_and_bids
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u32 + 1) * *bid)
        .sum()
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_hand_type(hand: &[usize], use_jokers: bool) -> HandType {
    let mut counts = [0_u32; 13];
    let mut joker_count = 0;
    for &card in hand {
        if use_jokers && card == 0 {
            joker_count += 1;
        } else {
            counts[card] += 1;
        }
    }
    let max_count = *counts.iter().max().unwrap() + joker_count;
    let pair_count = counts.iter().filter(|&&count| count == 2).count();
    use HandType::*;
    match max_count {
        5 => FiveOfAKind,
        4 => FourOfAKind,
        3 => {
            if (joker_count == 0 && pair_count == 1) || (joker_count == 1 && pair_count == 2) {
                FullHouse
            } else {
                ThreeOfAKind
            }
        }
        2 => {
            if pair_count == 2 {
                TwoPair
            } else {
                OnePair
            }
        }
        _ => HighCard,
    }
}

fn cmp_hands(a: &[usize], b: &[usize], use_jokers: bool) -> Ordering {
    let a_type = get_hand_type(a, use_jokers);
    let b_type = get_hand_type(b, use_jokers);
    if a_type != b_type {
        a_type.cmp(&b_type)
    } else {
        a.cmp(&b)
    }
}
