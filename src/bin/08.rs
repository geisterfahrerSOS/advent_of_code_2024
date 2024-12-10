advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let map = parse_map(input);
    let mut antinodes: Vec<(u32, u32)> = Vec::new();
    for (row_i, row) in map.iter().enumerate() {
        for (col_i, col) in row.iter().enumerate() {
            if col != &'.' {
                for (row_pair_i, row_pair) in map.iter().enumerate() {
                    for (col_pair_i, col_pair) in row_pair.iter().enumerate() {
                        if col_pair == col && row_i != row_pair_i && col_i != col_pair_i {
                            let distance = (
                                row_pair_i as i32 - row_i as i32,
                                col_pair_i as i32 - col_i as i32,
                            );
                            let point = (
                                row_pair_i as i32 + distance.0,
                                col_pair_i as i32 + distance.1,
                            );
                            println!("Point: {:?}", point);
                            if point.0 < map.len() as i32 && point.1 >= 0 as i32 {
                                if point.1 < map[0].len() as i32 && point.1 >= 0 as i32 {
                                    // second antinode correct
                                    println!("pushed +");
                                    if !antinodes.iter().any(|&x| x == (point.0 as u32, point.1 as u32)) {
                                        antinodes.push((point.0 as u32, point.1 as u32));
                                        println!("Point: {:?}", point);
                                    }
                                }
                            }
                            let point = (
                                row_i as i32 - distance.0,
                                row_i as i32 - distance.1,
                            );
                            println!("Point: {:?}", point);
                            if point.0 < map.len() as i32 && point.1 >= 0 as i32 {
                                if point.1 < map[0].len() as i32 && point.1 >= 0 as i32 {
                                    // second antinode correct
                                    println!("pushed -");
                                    if !antinodes.iter().any(|&x| x == (point.0 as u32, point.1 as u32)){
                                        antinodes.push((point.0 as u32, point.1 as u32));
                                        println!("Point: {:?}", point);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    for antinode in &antinodes {
        println!("{:?}", antinode);
    }
    Some(antinodes.len() as u32)
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    let map: Vec<Vec<char>> = input
        .split("\n")
        .filter(|&row| row != "")
        .map(|row| row.chars().collect())
        .collect();
    map
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
