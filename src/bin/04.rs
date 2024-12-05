use std::fmt::Error;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let xmas_matrix = create_xmas_matrix(input);
    let mut xmas_coordinates: Vec<XmasCoordinate> = Vec::new();
    let xmas_matrix_bounds = Bounds::from_matrix(&xmas_matrix);
    for row in 0..xmas_matrix.len() {
        for col in 0..xmas_matrix[row].len() {
            let master_coordinate = Coordinate {
                row: row as u32,
                col: col as u32,
            };
            match xmas_matrix[row][col] {
                'X' => {
                    let possible_directions =
                        search_around_coordinate(&xmas_matrix, 'M', &master_coordinate);
                    for direction in &possible_directions {
                        let mut current_coordinate = master_coordinate.clone();
                        let delta = direction_to_delta(&direction);
                        current_coordinate = match adjust_current_coordinate(
                            &current_coordinate,
                            &delta,
                            &xmas_matrix_bounds,
                        ) {
                            Ok(coordinate) => coordinate,
                            Err(_) => continue,
                        };
                        if search_in_direction(
                            &xmas_matrix,
                            direction,
                            'A',
                            &current_coordinate,
                            &xmas_matrix_bounds,
                        ) {
                            current_coordinate = match adjust_current_coordinate(
                                &current_coordinate,
                                &delta,
                                &xmas_matrix_bounds,
                            ) {
                                Ok(coordinate) => coordinate,
                                Err(_) => continue,
                            };
                            if search_in_direction(
                                &xmas_matrix,
                                direction,
                                'S',
                                &current_coordinate,
                                &xmas_matrix_bounds,
                            ) {
                                let xmas_coordinate = XmasCoordinate {
                                    direction: direction.clone(),
                                    coordinate: master_coordinate.clone(),
                                };
                                xmas_coordinates.push(xmas_coordinate);
                            } else {
                                continue;
                            }
                        } else {
                            continue;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    Some(xmas_coordinates.len() as u32)
}

fn create_xmas_matrix(input: &str) -> Vec<Vec<char>> {
    let rows: Vec<&str> = input.split('\n').collect();
    let mut xmas_matrix: Vec<Vec<char>> = Vec::new();
    for row in &rows {
        xmas_matrix.push(row.chars().collect())
    }
    if xmas_matrix[xmas_matrix.len() - 1] == [] {
        xmas_matrix.pop();
    }
    xmas_matrix
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Directions {
    Right,
    TopRight,
    Top,
    TopLeft,
    Left,
    BottomLeft,
    Bottom,
    BottomRight,
}

fn search_around_coordinate(
    xmas_matrix: &Vec<Vec<char>>,
    character: char,
    coordinate: &Coordinate,
) -> Vec<Directions> {
    let col: usize = coordinate.col as usize;
    let row: usize = coordinate.row as usize;
    let mut possible_directions: Vec<Directions> = Vec::new();
    if col < xmas_matrix[0].len() - 1 {
        if xmas_matrix[row][col + 1] == character {
            possible_directions.push(Directions::Right);
        }
        if row > 0 {
            if xmas_matrix[row - 1][col + 1] == character {
                possible_directions.push(Directions::TopRight);
            }
        }
        if row < xmas_matrix.len() - 1 {
            if xmas_matrix[row + 1][col + 1] == character {
                possible_directions.push(Directions::BottomRight);
            }
        }
    }
    if col > 0 {
        if xmas_matrix[row][col - 1] == character {
            possible_directions.push(Directions::Left);
        }
        if row > 0 {
            if xmas_matrix[row - 1][col - 1] == character {
                possible_directions.push(Directions::TopLeft);
            }
        }
        if row < xmas_matrix.len() - 1 {
            if xmas_matrix[row + 1][col - 1] == character {
                possible_directions.push(Directions::BottomLeft);
            }
        }
    }
    if row > 0 {
        if xmas_matrix[row - 1][col] == character {
            possible_directions.push(Directions::Top);
        }
    }
    if row < xmas_matrix.len() - 1 {
        if xmas_matrix[row + 1][col] == character {
            possible_directions.push(Directions::Bottom);
        }
    }
    possible_directions
}

fn search_in_direction(
    xmas_matrix: &Vec<Vec<char>>,
    direction: &Directions,
    character: char,
    coordinate: &Coordinate,
    bounds: &Bounds,
) -> bool {
    let delta = direction_to_delta(&direction);
    let read_coordinate = match adjust_current_coordinate(&coordinate, &delta, bounds) {
        Ok(co) => co,
        Err(_) => return false,
    };
    // println!("direction: {:?}   coordinate row: {}    delta_row: {}    coordinate col: {}   delta col: {}",direction, coordinate.row as i32, delta.row_delta, coordinate.col as i32, delta.col_delta);
    return read_matrix_by_coordinate(&xmas_matrix, &read_coordinate) == character;
}

#[derive(Clone, PartialEq, Debug)]
struct XmasCoordinate {
    coordinate: Coordinate,
    direction: Directions,
}

fn direction_to_delta(direction: &Directions) -> Delta {
    match direction {
        Directions::Right => return Delta::from_tuple((0, 1)),
        Directions::TopRight => return Delta::from_tuple((-1, 1)),
        Directions::Top => return Delta::from_tuple((-1, 0)),
        Directions::TopLeft => return Delta::from_tuple((-1, -1)),
        Directions::Left => return Delta::from_tuple((0, -1)),
        Directions::BottomLeft => return Delta::from_tuple((1, -1)),
        Directions::Bottom => return Delta::from_tuple((1, 0)),
        Directions::BottomRight => return Delta::from_tuple((1, 1)),
    }
}

fn adjust_current_coordinate(
    current_coordinate: &Coordinate,
    delta: &Delta,
    bounds: &Bounds,
) -> Result<Coordinate, Error> {
    let mut adjusted_coordinate: Coordinate = Coordinate { row: 0, col: 0 };
    if !bounds.is_inside(current_coordinate, delta) {
        return Err(Error);
    }
    adjusted_coordinate.row = (current_coordinate.row as i32 + delta.row_delta) as u32;
    adjusted_coordinate.col = (current_coordinate.col as i32 + delta.col_delta) as u32;
    return Ok(adjusted_coordinate);
}

#[derive(Clone, PartialEq, Debug)]
struct Coordinate {
    row: u32,
    col: u32,
}

#[derive(Clone, PartialEq, Debug)]
struct Delta {
    row_delta: i32,
    col_delta: i32,
}

impl Delta {
    fn from_tuple(delta_tuple: (i32, i32)) -> Self {
        Self {
            row_delta: delta_tuple.0,
            col_delta: delta_tuple.1,
        }
    }
}

struct Bounds {
    row_bounds: usize,
    col_bounds: usize,
}

impl Bounds {
    fn is_inside(&self, coordinate: &Coordinate, delta: &Delta) -> bool {
        if (coordinate.row as i32 + delta.row_delta) < 0
            || (coordinate.row as i32 + delta.row_delta) >= self.row_bounds as i32
        {
            return false;
        }
        if (coordinate.col as i32 + delta.col_delta) < 0
            || (coordinate.col as i32 + delta.col_delta) >= self.col_bounds as i32
        {
            return false;
        }
        return true;
    }
}

impl Bounds {
    fn from_matrix(matrix: &Vec<Vec<char>>) -> Self {
        Self {
            row_bounds: matrix.len(),
            col_bounds: matrix[0].len(),
        }
    }
}

fn read_matrix_by_coordinate(matrix: &Vec<Vec<char>>, coordinate: &Coordinate) -> char {
    return matrix[coordinate.row as usize][coordinate.col as usize];
}

pub fn part_two(input: &str) -> Option<u32> {
    let xmas_matrix = create_xmas_matrix(input);
    let mut xmas_coordinates: Vec<Coordinate> = Vec::new();
    for row in 0..xmas_matrix.len() as u32 {
        for col in 0..xmas_matrix[row as usize].len() as u32 {
            let master_coordinate = Coordinate { row: row, col: col };
            match read_matrix_by_coordinate(&xmas_matrix, &master_coordinate) {
                'A' => {
                    let possible_directions_m = filter_directions(search_around_coordinate(
                        &xmas_matrix,
                        'M',
                        &master_coordinate,
                    ));
                    let possible_directions_s = filter_directions(search_around_coordinate(
                        &xmas_matrix,
                        'S',
                        &master_coordinate,
                    ));
                    if possible_directions_m.len() != 2 || possible_directions_s.len() != 2 {
                        continue;
                    }
                    if determine_cross(&possible_directions_m, &possible_directions_s) {
                        xmas_coordinates.push(master_coordinate);
                    }
                }
                _ => {}
            }
        }
    }
    Some(xmas_coordinates.len() as u32)
}

fn determine_cross(directions_m: &Vec<Directions>, directions_s: &Vec<Directions>) -> bool {
    if &opposite_direction(&directions_m[0]) == &directions_m[1] {
        return false;
    }
    if &opposite_direction(&directions_s[0]) == &directions_s[1] {
        return false;
    }
    return true;
}

fn opposite_direction(direction: &Directions) -> Directions {
    return match direction {
        Directions::Right => Directions::Left,
        Directions::TopRight => Directions::BottomLeft,
        Directions::Top => Directions::Bottom,
        Directions::TopLeft => Directions::BottomRight,
        Directions::Left => Directions::Right,
        Directions::BottomLeft => Directions::TopRight,
        Directions::Bottom => Directions::Top,
        Directions::BottomRight => Directions::TopLeft,
    };
}

fn filter_directions(directions: Vec<Directions>) -> Vec<Directions> {
    return directions
        .into_iter()
        .filter(|d| {
            matches!(
                d,
                Directions::TopRight
                    | Directions::TopLeft
                    | Directions::BottomLeft
                    | Directions::BottomRight
            )
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
