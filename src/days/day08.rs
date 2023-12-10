use std::collections::HashMap;

use num::Integer;

use crate::{harness::input::RawInput, regex, util::re::parse_with_regex};

pub fn solve_part1(input: RawInput) -> u64 {
    let input = input.as_str();
    let (lrs, rest) = input.split_once("\n\n").unwrap();
    let lrs = lrs.as_bytes();
    let mut map = HashMap::<String, (String, String)>::new();
    for line in rest.lines() {
        let (a, b, c) =
            parse_with_regex::<(String, String, String)>(regex!(r"(.+) = \((.+), (.+)\)"), line)
                .unwrap();
        map.insert(a, (b, c));
    }
    let mut count = 0;
    let mut current = "AAA".to_owned();
    for i in 0.. {
        count += 1;
        let lr = lrs[i % lrs.len()];
        if lr == b'L' {
            current = map[&current].0.clone();
        } else {
            current = map[&current].1.clone();
        }
        if &current == "ZZZ" {
            return count;
        }
    }
    todo!()
}

pub fn solve_part2(input: RawInput) -> u64 {
    let input = input.as_str();
    let (lrs, rest) = input.split_once("\n\n").unwrap();
    let lrs = lrs.as_bytes();
    let mut map = HashMap::<String, (String, String)>::new();
    for line in rest.lines() {
        let (a, b, c) =
            parse_with_regex::<(String, String, String)>(regex!(r"(.+) = \((.+), (.+)\)"), line)
                .unwrap();
        map.insert(a, (b, c));
    }
    let aa = map
        .keys()
        .filter(|s| s.ends_with('A'))
        .cloned()
        .map(|s| {
            let mut count = 0;
            let mut current = s;
            for i in 0.. {
                count += 1;
                let lr = lrs[i % lrs.len()];
                if lr == b'L' {
                    current = map[&current].0.clone();
                } else {
                    current = map[&current].1.clone();
                }
                if current.ends_with('Z') {
                    return count;
                }
            }
            todo!()
        })
        .collect::<Vec<_>>();
    let mut result = aa[0];
    for &next in aa.iter().skip(1) {
        result = result.lcm(&next);
    }
    result

    // let mut currents = map
    //     .keys()
    //     .filter(|s| s.ends_with('A'))
    //     .cloned()
    //     .collect::<Vec<_>>();

    // let mut count = 0;
    // for i in 0.. {
    //     count += 1;
    //     let lr = lrs[i % lrs.len()];
    //     if lr == b'L' {
    //         currents = currents.into_iter().map(|s| map[&s].0.clone()).collect();
    //     } else {
    //         currents = currents.into_iter().map(|s| map[&s].1.clone()).collect();
    //     }
    //     if currents.iter().all(|s| s.ends_with('Z')) {
    //         return count;
    //     }
    // }
    // todo!()
}
