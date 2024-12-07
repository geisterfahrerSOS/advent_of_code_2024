use std::{collections::HashMap, fmt::Error, path::Component, vec};
use rand::seq::SliceRandom; // 0.7.2

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let parse_output = parse_rules_and_prints(input);
    let mut safe_prints: Vec<&Vec<u32>> = Vec::new();
    for print in &parse_output.prints {
        if check_print_safety(print, &parse_output.rules) {
            safe_prints.push(print);
        } else {
            continue;
        }
    }
    let middle_page_number_sum: u32 = safe_prints.iter().map(|print| print[print.len() / 2]).sum();
    Some(middle_page_number_sum)
}
struct ParseOutput {
    rules: Vec<(u32, u32)>,
    prints: Vec<Vec<u32>>,
}

fn parse_rules_and_prints(input: &str) -> ParseOutput {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut prints: Vec<Vec<u32>> = Vec::new();
    let mut section_toggle = false;
    for row in &rows {
        if row.is_empty() {
            section_toggle = true;
            continue;
        }
        if !section_toggle {
            let parts: Vec<&str> = row.split("|").collect();
            rules.push((parts[0].parse().unwrap(), parts[1].parse().unwrap()));
        } else {
            let print: Vec<u32> = row.split(",").map(|entry| entry.parse().unwrap()).collect();
            prints.push(print);
        }
    }
    return ParseOutput {
        rules: rules,
        prints: prints,
    };
}

fn check_rule(rules: &Vec<(u32, u32)>, primary: u32, secondary: u32) -> bool {
    if rules.iter().any(|&rule| rule == (primary, secondary)) {
        return true;
    } else if rules.iter().any(|&rule| rule == (secondary, primary)) {
        return false;
    }
    return true;
}

fn check_print_safety(print: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for index in 0..print.len() {
        for check_index in index + 1..print.len() {
            if !check_rule(rules, print[index], print[check_index]) {
                return false;
            }
        }
    }
    return true;
}

pub fn part_two(input: &str) -> Option<u32> {
    let parse_output = parse_rules_and_prints(input);
    let rule_topology = match transform_rules_to_topology_new(&parse_output.rules) {
        Ok(topology) => topology,
        Err(_) => Vec::new(),
    };

    // let mut correctly_ordered_prints: Vec<Vec<u32>> = Vec::new();
    // for print in &parse_output.prints {
    //     if !check_print_safety(print, &parse_output.rules) {
    //         correctly_ordered_prints.push(correct_print(print.clone(), &parse_output.rules));
    //     }
    // }
    // let middle_page_number_sum: u32 = correctly_ordered_prints
    //     .iter()
    //     .map(|print| print[print.len() / 2])
    //     .sum();
    // Some(middle_page_number_sum)
    None
}

fn correct_print(mut print: Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    for index in 0..print.len() {
        for check_index in index + 1..print.len() {
            let primary = print[index].clone();
            let secondary = print[check_index].clone();
            if !check_rule(rules, primary, secondary) {
                print[index] = secondary;
                print[check_index] = primary;
            }
        }
    }
    return print;
}
//Kahn's algorithm
fn transform_rules_to_topology(rules: &Vec<(u32, u32)>) -> Result<Vec<u32>, Error> {
    let mut rules = rules.clone();
    let mut starting_nodes: Vec<u32> = get_all_nodes(&rules)
        .into_iter()
        .filter(|&node| {
            return !rules.iter().any(|&rule| rule.1 == node) && rules.iter().any(|&rule| rule.0 == node);
        })
        .collect();
    println!("starting nodes: {:?}", starting_nodes);
    let mut sorted_nodes: Vec<u32> = Vec::new();
    while starting_nodes.len() > 0 {
        let current_node: u32 = starting_nodes.pop().unwrap();
        sorted_nodes.push(current_node);
        println!("outgoing from n: {:?}", find_rule(&rules, Some(current_node), None));

        for rule in find_rule(&rules, Some(current_node), None) {
            println!("rule: {:?}", rule);

            rules.remove(rules.iter().position(|&x| x == rule).unwrap());
            println!("urles: {:?}", rules);
            println!("incoming to m: {:?}", find_rule(&rules, None, Some(rule.1)));
            if find_rule(&rules, None, Some(rule.1)).len() == 0 {
                starting_nodes.push(rule.1);
            }
        }
    }
    println!("rules: {:?}", rules);
    println!("starting nodes: {:?}", starting_nodes);
    println!("sorted nodes: {:?}", sorted_nodes);

    Err(Error)
}

fn get_all_nodes(rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut all_nodes: Vec<u32> = Vec::new();
    rules.iter().for_each(|&rule| {
        if !all_nodes.iter().any(|&x| x == rule.0) {
            all_nodes.push(rule.0);
        }
        if !all_nodes.iter().any(|&x| x == rule.1) {
            all_nodes.push(rule.1);
        }
    });
    all_nodes
}

//depth-first-search algorithm
fn transform_rules_to_topology_new(rules: &Vec<(u32, u32)>) -> Result<Vec<u32>, Error> {
    let mut marks: Vec<u32> = Vec::new();
    let rules = rules.clone();
    let mut sorted_nodes: Vec<u32> = Vec::new();
    let nodes = get_all_nodes(&rules);
    while marks.len() < nodes.len() {
        let current_node: u32 = *nodes.choose(&mut rand::thread_rng()).unwrap();
        visit(current_node, &mut marks, &mut sorted_nodes, &rules);
    }
    println!("rules: {:?}", rules);
    println!("sorted nodes: {:?}", sorted_nodes);

    Err(Error)
}

fn visit(node: u32, marks: &mut Vec<u32>, sorted_nodes: &mut Vec<u32>, rules: &Vec<(u32, u32)>) {
    if marks.iter().any(|&x| x == node) {
        return
    }
    marks.push(node);
    for rule in find_rule(&rules, Some(node), None) {
        visit(rule.1, marks, sorted_nodes, rules);
    }
    marks.push(node);
    sorted_nodes.push(node);

}



fn find_rule(
    rules: &Vec<(u32, u32)>,
    primary: Option<u32>,
    secondary: Option<u32>,
) -> Vec<(u32, u32)> {
    match primary {
        Some(first) => match secondary {
            Some(second) => {
                return rules
                    .into_iter()
                    .filter(|&&rule| rule == (first, second))
                    .cloned()
                    .collect();
            }
            None => {
                return rules
                    .iter()
                    .filter(|&&rule| rule.0 == first)
                    .cloned()
                    .collect();
            }
        },
        None => match secondary {
            Some(second) => {
                return rules
                    .into_iter()
                    .filter(|&&rule| rule.1 == second)
                    .cloned()
                    .collect();
            }
            None => return vec![],
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
