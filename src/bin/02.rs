advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports = generate_reports(input);
    let mut safe_count = 0;
    for report in &reports {
        if determine_safety(report) {
            safe_count = safe_count + 1;
        }
    }
    Some(safe_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = generate_reports(input);
    let mut safe_count = 0;
    for report in &reports {
        if determine_safety(report) {
            safe_count = safe_count + 1;
        } else {
            if brute_force_to_safety(report) {
                safe_count = safe_count + 1;
            }
        }
    }
    Some(safe_count)
}

fn generate_reports(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|row| row.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect()
}

fn brute_force_to_safety(report: &Vec<u32>) -> bool {
    for index in 0..(report.len()) {
        let mut modified_report = report.clone();
        modified_report.remove(index);
        if determine_safety(&modified_report) {
            return true;
        }
    }
    false
}

fn determine_safety(report: &Vec<u32>) -> bool {
    let mut safe = true;
    let rising = if (report[1] as i32) - (report[0] as i32) > 0 {
        true
    } else {
        false
    };
    for i in 0..(report.len() - 1) {
        let delta = (report[i + 1] as i32) - (report[i] as i32);
        if delta.abs() < 1 || delta.abs() > 3 {
            safe = false;
            break;
        }
        if rising != (delta > 0) {
            safe = false;
            break;
        }
    }
    safe
}

fn safety_changes(report: &mut Vec<u32>) -> bool {
    let mut delta_report: Vec<i32> = Vec::new();
    for i in 0..(report.len() - 1) {
        delta_report.push((report[i + 1] as i32) - (report[i] as i32));
    }
    println!("{:?}", delta_report);
    let same_sign = delta_report.iter().all(|&x| x > 0)
        || delta_report.iter().all(|&x| x == 0)
        || delta_report.iter().all(|&x| x < 0);
    if same_sign {
        return false;
    }
    let positive = delta_report.iter().filter(|&&x| x > 0).count();
    println!("pos {}", positive);
    let negative = delta_report.iter().filter(|&&x| x < 0).count();
    println!("neg {}", negative);
    if positive == (delta_report.len() - 1) {
        report.remove(delta_report.iter().position(|&x| x <= 0).unwrap());
        return true;
    }
    if negative == (delta_report.len() - 1) {
        report.remove(delta_report.iter().position(|&x| x >= 0).unwrap() + 1);
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
