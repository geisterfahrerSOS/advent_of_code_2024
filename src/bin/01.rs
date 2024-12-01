advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let number_rows: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let mut cols: (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
    for row in &number_rows {
        let parts: Vec<u32> = row.split("   ").map(|x| x.parse::<u32>().unwrap()).collect();
        if parts.len() >= 2 {
            cols.0.push(parts[0]);
            cols.1.push(parts[1]);
        }
    }
    cols.0.sort();
    cols.1.sort();
    let cols: Vec<(u32, u32)> = cols.0.iter().zip(cols.1.iter()).map(|(&a, &b)| (a, b)).collect();
    let mut delta_distance: Vec<u32> = Vec::new();
    for row in &cols {
        delta_distance.push(if  row.0 > row.1 {row.0- row.1} else {row.1 - row.0});
    }
    let total_distance: Option<u32> = Some(delta_distance.iter().sum());

    total_distance
}

pub fn part_two(input: &str) -> Option<u32> {
    let number_rows: Vec<&str> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let mut cols: (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
    for row in &number_rows {
        let parts: Vec<u32> = row.split("   ").map(|x| x.parse::<u32>().unwrap()).collect();
        if parts.len() >= 2 {
            cols.0.push(parts[0]);
            cols.1.push(parts[1]);
        }
    }
    cols.0.sort();
    cols.1.sort();
    let mut second_number_grouped: Vec<(u32, u32)> = Vec::new();
    let mut current_number: u32 = 0;
    let mut count: u32 = 0;
    for number in cols.1 {
        if number != current_number {
            second_number_grouped.push((current_number, count));
            current_number = number;
            count = 0;
        }
        count = count + 1;
    }

    let mut similarity: u32 = 0;
    for number in cols.0 {
        match second_number_grouped.iter().find(|&&x| x.0 == number) {
            Some((number, amount)) => similarity = similarity + number * amount,
            None => (),
        }
    }
    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
