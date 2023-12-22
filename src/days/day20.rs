use std::collections::{HashMap, HashSet, VecDeque};

use arrayvec::ArrayString;

use crate::{harness::input::RawInput, util::search::bfs};

type Name = ArrayString<3>;

#[derive(Debug)]

enum ModuleState {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<Name, bool>),
}

use ModuleState::*;

#[derive(Debug)]
struct Module {
    state: ModuleState,
    outs: Vec<Name>,
}

pub fn solve_part1(input: RawInput) -> usize {
    // let mut modules_by_name = parse_modules(input.as_str());
    // let mut low_pulse_count = 0;
    // let mut high_pulse_count = 0;
    // for _ in 0..1000 {
    //     let (low, high) = run_one_round(&mut modules_by_name);
    //     low_pulse_count += low;
    //     high_pulse_count += high;
    // }
    // low_pulse_count * high_pulse_count
    todo!()
}

pub fn solve_part2(input: RawInput) -> usize {
    let mut modules_by_name = parse_modules(input.as_str());
    let mut count = 0;
    loop {
        let aa = run_one_round(&mut modules_by_name, Name::from("nj").unwrap());
        if let Some(signal) = aa {
            if signal {
                println!("{count} {signal:?}");
            }
        }

        count += 1;
    }
}

fn parse_modules(s: &str) -> HashMap<Name, Module> {
    let mut ins_by_name = HashMap::<Name, Vec<Name>>::new();
    let mut modules_by_name = s
        .lines()
        .map(|line| {
            let (label, outs) = line.split_once(" -> ").unwrap();
            let state = match label.as_bytes()[0] {
                b'b' => Broadcaster,
                b'%' => FlipFlop(false),
                b'&' => Conjunction(HashMap::new()),
                _ => panic!("what"),
            };
            let name = Name::from(&label[1..4.min(label.len())]).unwrap();
            let outs = outs
                .split(", ")
                .map(|s| Name::from(&s[..3.min(s.len())]).unwrap())
                .collect::<Vec<_>>();
            for &out in &outs {
                ins_by_name.entry(out).or_default().push(name);
            }
            (name, Module { state, outs })
        })
        .collect::<HashMap<_, _>>();
    for (&name, module) in modules_by_name.iter_mut() {
        if let Conjunction(remembered_ins) = &mut module.state {
            *remembered_ins = ins_by_name[&name]
                .iter()
                .map(|&in_name| (in_name, false))
                .collect();
        }
    }
    modules_by_name
}

fn run_one_round(modules_by_name: &mut HashMap<Name, Module>, out: Name) -> Option<bool> {
    type Signal = (Name, Name, bool);
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    let mut pending_signals = VecDeque::new();
    pending_signals.push_back((Name::from("").unwrap(), Name::from("roa").unwrap(), false));
    let mut result = None;
    while let Some((from, to, signal)) = pending_signals.pop_front() {
        if from == out && signal {
            result = Some(true);
        }
        if signal {
            high_pulse_count += 1;
        } else {
            low_pulse_count += 1;
        }
        let module = modules_by_name.get_mut(&to);
        let Some(module) = module else {
            continue;
        };
        let next_signal = match &mut module.state {
            Broadcaster => Some(signal),
            FlipFlop(state) => {
                if !signal {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            }
            Conjunction(state) => {
                state.insert(from, signal);
                Some(state.values().any(|&remembered| !remembered))
            }
        };
        if let Some(next_signal) = next_signal {
            for &out in &module.outs {
                pending_signals.push_back((to, out, next_signal));
            }
        }
    }
    result
    // false
    // (low_pulse_count, high_pulse_count)
}
