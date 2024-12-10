use std::fmt::format;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_equations(input);
    let mut sum_of_possible_equation_results: u64 = 0;
    for equation in &equations {
        'inner: for operation_representation in 0..2_u32.pow((equation.1.len() - 1) as u32) {
            let mut result: u64 = equation.1[0] as u64;
            // println!("Start result: {}", result);
            // println!("Operation representation: {:b}", operation_representation);
            for calculation_counter in 0..(equation.1.len() - 1) {
                // println!("Calculation counter: {}", calculation_counter);
                if (operation_representation >> (equation.1.len() - 2 - calculation_counter)) & 0b1
                    == 1
                {
                    // println!("*: {}", equation.1[calculation_counter + 1]);
                    result *= equation.1[calculation_counter + 1] as u64;
                } else {
                    // println!("+: {}", equation.1[calculation_counter + 1]);
                    result += equation.1[calculation_counter + 1] as u64;
                }
                if result > equation.0 {
                    break;
                }
            }
            // println!("Result: {}", result);
            if result == equation.0 {
                // println!("correct");
                sum_of_possible_equation_results += result;
                break 'inner;
            }
        }
    }
    Some(sum_of_possible_equation_results as u64)
}

fn parse_equations(input: &str) -> Vec<(u64, Vec<u64>)> {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut equations: Vec<(u64, Vec<u64>)> = Vec::new();
    for row in rows {
        if row == "" {
            continue;
        }
        let splitrow: Vec<&str> = row.split(": ").collect();
        equations.push((
            splitrow[0].parse().unwrap(),
            splitrow[1]
                .split(" ")
                .map(|value| value.parse().unwrap())
                .collect(),
        ));
    }
    equations
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_equations(input);
    let mut sum_of_possible_equation_results: u64 = 0;
    for equation in &equations {
        'inner: for operation_representation in 0..2_u32.pow(2 * (equation.1.len() - 1) as u32) {
            let mut result: u64 = equation.1[0] as u64;
            println!("Start result: {}", result);
            println!(
                "Operation representation: {:0width$b}",
                operation_representation,
                width = 2 * (equation.1.len() - 1)
            );
            for calculation_counter in 0..(equation.1.len() - 1) {
                println!("Calculation counter: {}", calculation_counter);
                match operation_representation >> 2 * (equation.1.len() - 2 - calculation_counter)
                    & 0b11
                {
                    0 => {
                        println!("+: {}", equation.1[calculation_counter + 1]);
                        result += equation.1[calculation_counter + 1] as u64;
                    }
                    1 => {
                        println!("*: {}", equation.1[calculation_counter + 1]);
                        result *= equation.1[calculation_counter + 1] as u64;
                    }
                    2 => {
                        println!("|{}", equation.1.len() - 2 - calculation_counter);
                        if (equation.1.len() - 2 - calculation_counter <= 0) {
                            continue;
                        }
                        match operation_representation
                            >> (2 * (equation.1.len() - 2 - calculation_counter - 1))
                            & 0b11
                        {
                            // get prior operation
                            0 => {
                                println!("2: +: {}", equation.1[calculation_counter + 1]);
                                result -= equation.1[calculation_counter] as u64;
                                result += format!(
                                    "{}{}",
                                    equation.1[calculation_counter],
                                    equation.1[calculation_counter + 1]
                                )
                                .parse::<u64>()
                                .unwrap();
                            }
                            1 => {
                                println!("2: *: {}", equation.1[calculation_counter + 1]);
                                result /= equation.1[calculation_counter] as u64;
                                result *= format!(
                                    "{}{}",
                                    equation.1[calculation_counter],
                                    equation.1[calculation_counter + 1]
                                )
                                .parse::<u64>()
                                .unwrap();
                            }
                            2 => {
                                println!("2: |: {}", equation.1[calculation_counter + 1]);

                                // let add_string = format!(
                                //     "{}{}",
                                //     equation.1[calculation_counter],
                                //     equation.1[calculation_counter + 1]
                                // );
                                // match_function(
                                //     &mut result,
                                //     operation_representation as u64,
                                //     equation,
                                //     calculation_counter as u64,
                                //     add_string,
                                // );
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }
                if result > equation.0 {
                    break;
                }
            }
            println!("Result: {}", result);
            if result == equation.0 {
                println!("correct");
                sum_of_possible_equation_results += result;
                break 'inner;
            }
        }
    }
    Some(sum_of_possible_equation_results as u64)
}

fn match_function(
    result: &mut u64,
    operation_representation: u64,
    equation: &(u64, Vec<u64>),
    calculation_counter: u64,
    add_string: String,
) {
    match operation_representation >> 2 * (equation.1.len() - 2 - calculation_counter as usize - 1)
        & 0b11
    {
        // get prior operation
        0 => {
            // println!("+: {}", equation.1[calculation_counter + 1]);
            *result -= equation.1[calculation_counter as usize] as u64;
            *result += format!(
                "{}{}",
                equation.1[calculation_counter as usize],
                equation.1[calculation_counter as usize + 1]
            )
            .parse::<u64>()
            .unwrap();
        }
        1 => {
            // println!("*: {}", equation.1[calculation_counter + 1]);
            *result /= equation.1[calculation_counter as usize] as u64;
            *result *= format!("{}{}", equation.1[calculation_counter as usize], add_string)
                .parse::<u64>()
                .unwrap();
        }
        2 => {
            match_function(
                result,
                operation_representation,
                equation,
                calculation_counter - 1,
                format!("{}{}", equation.1[calculation_counter as usize], add_string),
            );
        }
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
