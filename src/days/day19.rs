use std::collections::HashMap;

use arrayvec::ArrayString;

use crate::{
    harness::input::RawInput,
    regex,
    util::{re::parse_with_regex, search::bfs},
};

type Name = ArrayString<3>;

#[derive(Debug, Clone)]
struct Workflow {
    name: Name,
    rules: Vec<Rule>,
    elsewise: Name,
}

#[derive(Debug, Copy, Clone)]
struct Rule {
    var_index: usize,
    is_less_than: bool,
    limit: usize,
    dest: Name,
}

pub fn solve_part1(input: RawInput) -> usize {
    let (workflows, parts) = input.as_str().split_once("\n\n").unwrap();
    let workflows_by_name = workflows
        .lines()
        .map(|line| parse_workflow(line))
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect::<HashMap<_, _>>();
    parts
        .lines()
        .map(parse_part)
        .filter(|part| {
            let mut current = Name::from("in").unwrap();
            loop {
                if &current == "A" {
                    return true;
                }
                if &current == "R" {
                    return false;
                }
                let workflow = &workflows_by_name[&current];
                let mut did_hit = false;
                for rule in &workflow.rules {
                    let value = part[rule.var_index];
                    if (rule.is_less_than && value < rule.limit)
                        || (!rule.is_less_than && value > rule.limit)
                    {
                        current = rule.dest;
                        did_hit = true;
                        break;
                    }
                }
                if !did_hit {
                    current = workflow.elsewise;
                }
            }
        })
        .map(|part| part.into_iter().sum::<usize>())
        .sum()
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    workflow_name: Name,
    limits: [[usize; 2]; 4], // inclusive
}

pub fn solve_part2(input: RawInput) -> usize {
    let (workflows, _) = input.as_str().split_once("\n\n").unwrap();
    let workflows_by_name = workflows
        .lines()
        .map(|line| parse_workflow(line))
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect::<HashMap<_, _>>();

    let initial_state = State {
        workflow_name: Name::from("in").unwrap(),
        limits: [[1, 4000]; 4],
    };
    let search_result = bfs::search(
        initial_state,
        |state| {
            if &state.workflow_name == "A" || &state.workflow_name == "R" {
                return vec![];
            }
            let workflow = &workflows_by_name[&state.workflow_name];
            let mut nexts = vec![];
            let mut limits = state.limits;
            let mut did_break = false;
            for rule in &workflow.rules {
                let index = rule.var_index;
                let mut next = State {
                    workflow_name: rule.dest.clone(),
                    limits: limits,
                };
                if rule.is_less_than {
                    next.limits[index][1] = next.limits[index][1].min(rule.limit - 1);
                    limits[index][0] = limits[index][0].max(rule.limit);
                } else {
                    next.limits[index][0] = next.limits[index][0].max(rule.limit + 1);
                    limits[index][1] = limits[index][1].min(rule.limit);
                }
                if next.limits[index][0] <= next.limits[index][1] {
                    nexts.push(next);
                }
                if limits[index][0] > limits[index][1] {
                    did_break = true;
                    break;
                }
            }
            if !did_break {
                nexts.push(State {
                    workflow_name: workflow.elsewise.clone(),
                    limits,
                });
            }
            nexts
        },
        |_| false,
    );
    search_result
        .seen_states
        .into_iter()
        .map(|seen| seen.state)
        .filter(|state| &state.workflow_name == "A")
        .map(|state| {
            state
                .limits
                .into_iter()
                .map(|[low, high]| high - low + 1)
                .product::<usize>()
        })
        .sum()
}

fn parse_workflow(s: &str) -> Workflow {
    let (name, rules) = parse_with_regex::<(Name, String)>(regex!(r"(.+)\{(.+)\}"), s).unwrap();
    let rule_strs = rules.split(',').collect::<Vec<_>>();
    let rules = rule_strs[..(rule_strs.len() - 1)]
        .iter()
        .map(|&s| {
            let (var, dir, limit, dest) =
                parse_with_regex::<(char, char, usize, Name)>(regex!(r"(.+)([<>])(\d+):(.+)"), s)
                    .unwrap();
            let is_less_than = dir == '<';
            let var_index = "xmas".find(var).unwrap();
            Rule {
                var_index,
                is_less_than,
                limit,
                dest,
            }
        })
        .collect();
    let elsewise = Name::from(rule_strs[rule_strs.len() - 1]).unwrap();
    Workflow {
        name,
        rules,
        elsewise,
    }
}

fn parse_part(s: &str) -> [usize; 4] {
    let (x, m, a, s) = parse_with_regex::<(usize, usize, usize, usize)>(
        regex!(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}"),
        s,
    )
    .unwrap();
    [x, m, a, s]
}
