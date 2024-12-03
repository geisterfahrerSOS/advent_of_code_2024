advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum_of_products: u32 = 0;
    let mul_indices = get_indices(input, "mul(");
    let corrupted_program_code: Vec<char> = input.chars().collect();

    for i in 0..mul_indices.len() {
        let mut products: (u32, u32) = (0, 0);
        let mul_slice: Vec<char>;
        mul_slice = if i == (mul_indices.len() - 1) {
            corrupted_program_code[mul_indices[i]..].to_vec()
        } else {
            corrupted_program_code[mul_indices[i]..mul_indices[i + 1]].to_vec()
        };

        let colon_index = mul_slice.iter().position(|&x| x == ',');
        match colon_index {
            Some(index) => {
                let parsing_result = mul_slice[4..index]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>();

                    match parsing_result {
                        Ok(number) => products.0 = if number < 1000 { number } else { continue }, Err(_) => continue,
                    }
                let bracket_index = mul_slice.iter().position(|&x| x == ')');
                match bracket_index {
                    Some(b_index) => {
                        let parsing_result = mul_slice[(index + 1)..b_index]
                            .iter()
                            .collect::<String>()
                            .parse::<u32>();
                        match parsing_result {
                            Ok(number) => products.1 = if number < 1000 { number } else { continue }, Err(_) => continue,
                        }
                    }
                    None => continue,
                }
            }
            None => continue,
        }
        sum_of_products += products.0 * products.1;
    }
    Some(sum_of_products)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum_of_products: u32 = 0;
    let mul_indices = get_indices(input, "mul(");
    let dont_indices = get_indices(input, "don't()");
    let do_indices = get_indices(input, "do()");
    let corrupted_program_code: Vec<char> = input.chars().collect();
    let forbiden_zone = create_forbidden_zone(do_indices, dont_indices, corrupted_program_code.len());

    for i in 0..mul_indices.len() {
        if forbiden_zone[mul_indices[i]] {
            continue;
        }
        let mut products: (u32, u32) = (0, 0);
        let mul_slice: Vec<char>;
        mul_slice = if i == (mul_indices.len() - 1) {
            corrupted_program_code[mul_indices[i]..].to_vec()
        } else {
            corrupted_program_code[mul_indices[i]..mul_indices[i + 1]].to_vec()
        };

        let colon_index = mul_slice.iter().position(|&x| x == ',');
        match colon_index {
            Some(index) => {
                let parsing_result = mul_slice[4..index]
                    .iter()
                    .collect::<String>()
                    .parse::<u32>();

                    match parsing_result {
                        Ok(number) => products.0 = if number < 1000 { number } else { continue }, Err(_) => continue,
                    }
                let bracket_index = mul_slice.iter().position(|&x| x == ')');
                match bracket_index {
                    Some(b_index) => {
                        let parsing_result = mul_slice[(index + 1)..b_index]
                            .iter()
                            .collect::<String>()
                            .parse::<u32>();
                        match parsing_result {
                            Ok(number) => products.1 = if number < 1000 { number } else { continue }, Err(_) => continue,
                        }
                    }
                    None => continue,
                }
            }
            None => continue,
        }
        sum_of_products += products.0 * products.1;
    }
    Some(sum_of_products)
}

fn get_indices(input: &str, pattern: &str) -> Vec<usize> {
    input.match_indices(pattern).map(|(index, _)| index).collect()
}

fn create_forbidden_zone(dos: Vec<usize>, donts: Vec<usize>, length: usize) -> Vec<bool> {
    let mut forbidden_zone: Vec<bool> = Vec::new();
    let mut forbidden = false;
    for i in 0..length {
        if donts.iter().any(|&x| x == i) {
            forbidden = true;
        }
        if dos.iter().any(|&x| x == i){
            forbidden = false;
        }
        forbidden_zone.push(forbidden);
    }
    forbidden_zone
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
