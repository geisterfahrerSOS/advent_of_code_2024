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
    let mut correctly_ordered_prints: Vec<Vec<u32>> = Vec::new();
    for print in &parse_output.prints {
        if !check_print_safety(print, &parse_output.rules) {
            correctly_ordered_prints.push(correct_print(print.clone(), &parse_output.rules));
        }
    }
    let middle_page_number_sum: u32 = correctly_ordered_prints.iter().map(|print| print[print.len() / 2]).sum();
    Some(middle_page_number_sum)
}

fn correct_print(mut print: Vec<u32>, rules:&Vec<(u32, u32)>) -> Vec<u32>{
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
    return print
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
