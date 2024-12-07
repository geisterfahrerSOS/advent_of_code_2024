use std::io::ErrorKind;

use std::io::Error;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut guard = parse_map_and_guard(input);
    let mut field_count: u32 = 1;
    loop {
        match guard.update() {
            GuardStates::Finished => break,
            GuardStates::Moved => field_count += 1,
            GuardStates::Rotated => (),
            GuardStates::MovedOverPath => (),
        }
    }
    Some(field_count)
}

fn parse_map_and_guard(input: &str) -> Guard {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut map: Map = Map {
        map: Vec::new(),
        obstacles: Vec::new(),
    };
    let mut guard = Guard {
        coordinate: Coordinate { row: 0, col: 0 },
        direction: Direction::Up,
        map: Map {
            map: Vec::new(),
            obstacles: Vec::new(),
        },
        history: Vec::new(),
    };
    for (index_row, row) in rows.iter().enumerate() {
        if row == &"" {
            continue;
        }
        map.map.push(
            row.chars()
                .enumerate()
                .map(|(index_col, field)| match field {
                    '.' => return FieldType::Open,
                    '<' | '>' | '^' | 'v' => {
                        guard = Guard {
                            coordinate: Coordinate {
                                row: index_row as i32,
                                col: index_col as i32,
                            },
                            direction: match field {
                                '<' => Direction::Left,
                                '>' => Direction::Right,
                                '^' => Direction::Up,
                                'v' => Direction::Down,
                                _ => unreachable!(),
                            },
                            map: Map {
                                map: Vec::new(),
                                obstacles: Vec::new(),
                            },
                            history: Vec::new(),
                        };
                        return FieldType::Open;
                    }
                    '#' => return FieldType::Obstruction,
                    _ => return FieldType::Open,
                })
                .collect(),
        );
    }
    guard.map = map;
    guard.history.push((guard.coordinate, guard.direction));
    guard
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FieldType {
    Obstruction,
    Open,
    OutOfBounds,
}
#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<FieldType>>,
    obstacles: Vec<Coordinate>,
}

impl Map {
    fn get_bounds(&self) -> Bounds {
        return Bounds {
            row_bounds: self.map.len() as u32,
            col_bounds: if self.map.len() > 0 {
                self.map[0].len() as u32
            } else {
                0
            },
        };
    }
}

impl Map {
    fn get_field_from_coordinate(&self, coordinate: &Coordinate) -> FieldType {
        if self.get_bounds().is_inside(coordinate) {
            return self.map[coordinate.row as usize][coordinate.col as usize];
        } else {
            return FieldType::OutOfBounds;
        }
    }
    fn set_field_from_coordinate(
        &mut self,
        coordinate: &Coordinate,
        field: FieldType,
    ) -> Result<FieldType, Error> {
        if self.get_bounds().is_inside(coordinate) {
            self.map[coordinate.row as usize][coordinate.col as usize] = field;
            return Ok(field);
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to set field"));
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Bounds {
    row_bounds: u32,
    col_bounds: u32,
}

impl Bounds {
    fn is_inside(&self, coordinate: &Coordinate) -> bool {
        if (coordinate.row) < 0 || (coordinate.row) >= self.row_bounds as i32 {
            return false;
        }
        if (coordinate.col) < 0 || (coordinate.col) >= self.col_bounds as i32 {
            return false;
        }
        return true;
    }
}

#[derive(Clone, PartialEq, Debug, Copy)]
struct Coordinate {
    row: i32,
    col: i32,
}

impl Coordinate {
    fn add_delta(&mut self, row_delta: i32, col_delta: i32) {
        self.row += row_delta;
        self.col += col_delta;
    }
    fn add_delta_with_direction(coordiante: &Coordinate, direction: &Direction) -> Self {
        let delta = direction.convert_to_delta();
        return Coordinate {
            row: coordiante.row + delta.0,
            col: coordiante.col + delta.1,
        };
    }
}

impl Direction {
    fn convert_to_delta(&self) -> (i32, i32) {
        match self {
            Direction::Right => return (0, 1),
            Direction::Up => return (-1, 0),
            Direction::Left => return (0, -1),
            Direction::Down => return (1, 0),
        }
    }
    fn turn_right(&self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
        }
    }
}
#[derive(Debug)]
struct Guard {
    coordinate: Coordinate,
    direction: Direction,
    map: Map,
    history: Vec<(Coordinate, Direction)>,
}

enum GuardStates {
    Finished,
    Moved,
    MovedOverPath,
    Rotated,
}

enum RunnerStates {
    Finished,
    Rotated,
    Moved,
    Overlapped,
}

impl Guard {
    fn update(&mut self) -> GuardStates {
        let next_coordinate =
            Coordinate::add_delta_with_direction(&self.coordinate, &self.direction);
        match self.map.get_field_from_coordinate(&next_coordinate) {
            FieldType::Obstruction => {
                self.direction = self.direction.turn_right();
                return GuardStates::Rotated;
            }
            FieldType::Open => {
                self.coordinate = next_coordinate;
                if self
                    .history
                    .iter()
                    .find(|&c| c.0 == self.coordinate)
                    .is_none()
                {
                    self.history.push((self.coordinate, self.direction));
                    return GuardStates::Moved;
                } else {
                    self.coordinate = next_coordinate;
                    return GuardStates::MovedOverPath;
                }
            }
            FieldType::OutOfBounds => return GuardStates::Finished,
        }
    }
    fn update_obstacle(&mut self) -> GuardStates {
        let next_coordinate =
            Coordinate::add_delta_with_direction(&self.coordinate, &self.direction);
        match self.map.get_field_from_coordinate(&next_coordinate) {
            FieldType::Obstruction => {
                self.direction = self.direction.turn_right();
                return GuardStates::Rotated;
            }
            FieldType::Open => {
                if check_for_possible_obstacle_with_runner(
                    self.coordinate,
                    self.direction,
                    self.map.clone(),
                ) {
                    self.map.obstacles.push(next_coordinate);
                }
                self.coordinate = next_coordinate;
                if self
                    .history
                    .iter()
                    .find(|&c| c.0 == self.coordinate)
                    .is_none()
                {
                    self.history.push((self.coordinate, self.direction));
                    return GuardStates::Moved;
                } else {
                    self.coordinate = next_coordinate;
                    return GuardStates::MovedOverPath;
                }
            }
            FieldType::OutOfBounds => return GuardStates::Finished,
        }
    }

    fn update_obstacle_runner(&mut self) -> RunnerStates {
        // println!("runner  running: {:#?}", self.coordinate);
        let next_coordinate =
            Coordinate::add_delta_with_direction(&self.coordinate, &self.direction);
        match self.map.get_field_from_coordinate(&next_coordinate) {
            FieldType::Obstruction => {
                self.direction = self.direction.turn_right();
                return RunnerStates::Rotated;
            }
            FieldType::Open => {
                self.coordinate = next_coordinate;
                if self
                    .history
                    .iter()
                    .any(|&history_item| history_item == (self.coordinate, self.direction))
                {
                    // println!("history: {:?}", self.history);
                    // println!("coord: {:?} dir: {:?}", self.coordinate, self.direction);
                    return RunnerStates::Overlapped;
                }
                self.history.push((self.coordinate, self.direction));
                return RunnerStates::Moved;
            }
            FieldType::OutOfBounds => return RunnerStates::Finished,
        }
    }

    fn check_for_possible_obstacle(&mut self) -> bool {
        if self
            .history
            .iter()
            .filter(|history_item| {
                history_item.1 == self.direction.turn_right()
                    && (((self.direction == Direction::Right || self.direction == Direction::Left)
                        && self.coordinate.col == history_item.0.col)
                        || ((self.direction == Direction::Up || self.direction == Direction::Down)
                            && self.coordinate.row == history_item.0.row))
            })
            .collect::<Vec<_>>()
            .len()
            > 0
        {
            self.map
                .obstacles
                .push(Coordinate::add_delta_with_direction(
                    &self.coordinate,
                    &self.direction,
                ));
            return true;
        }
        return false;
    }
}
fn check_for_possible_obstacle_with_runner(
    coordinate: Coordinate,
    direction: Direction,
    map: Map,
) -> bool {
    let mut map = map.clone();
    let _ = map.set_field_from_coordinate(
        &Coordinate::add_delta_with_direction(&coordinate, &direction),
        FieldType::Obstruction,
    );
    let mut guard = Guard {
        map: map,
        direction: direction.turn_right(),
        history: Vec::new(),
        coordinate: coordinate,
    };
    // println!("runner started at: {:#?}", guard.coordinate);
    loop {
        match guard.update_obstacle_runner() {
            RunnerStates::Finished => return false,
            RunnerStates::Moved => (),
            RunnerStates::Rotated => (),
            RunnerStates::Overlapped => return true,
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut guard = parse_map_and_guard(input);
    loop {
        match guard.update_obstacle() {
            GuardStates::Finished => break,
            GuardStates::Moved => (),
            GuardStates::Rotated => (),
            GuardStates::MovedOverPath => (),
        }
    }
    println!("{:?}", guard.map.obstacles);
    // println!("{:?}", guard.history);
    Some(guard.map.obstacles.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
