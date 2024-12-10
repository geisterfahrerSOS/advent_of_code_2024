advent_of_code::solution!(10);

use std::{collections::{hash_set, HashSet}, io::{Error, ErrorKind}};

fn parse_map(input: &str) -> Map {
    println!("hi");
    let rows: Vec<&str> = input.split("\n").collect();
    let mut map: Map = Map { map: Vec::new() };
    for row in rows.iter() {
        if row == &"" {
            continue;
        }
        map.map.push(
            row.chars()
                .map(|entry| entry.to_string().parse::<u32>().unwrap())
                .collect(),
        );
    }
    map
}

fn find_possible_trail_heads(map: &Map) -> Vec<Coordinate> {
    let mut possible_trail_heads = Vec::new();
    for row in map.map.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            if col.1 == &0 {
                possible_trail_heads.push(Coordinate {
                    row: row.0 as i32,
                    col: col.0 as i32,
                })
            }
        }
    }
    possible_trail_heads
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<u32>>,
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
    fn get_height_from_coordinate(&self, coordinate: &Coordinate) -> u32 {
        if self.get_bounds().is_inside(coordinate) {
            return self.map[coordinate.row as usize][coordinate.col as usize];
        } else {
            return 10;
        }
    }
    fn set_field_from_coordinate(
        &mut self,
        coordinate: &Coordinate,
        field: u32,
    ) -> Result<u32, Error> {
        if self.get_bounds().is_inside(coordinate) {
            self.map[coordinate.row as usize][coordinate.col as usize] = field;
            return Ok(field);
        } else {
            return Err(Error::new(ErrorKind::Other, "Failed to set field"));
        }
    }
    // returns vector of directions matching the number
    fn search_around(&self, coordinate: &Coordinate, height: u32) -> Vec<Direction> {
        let mut directions: Vec<Direction> = Vec::new();
        for direction in Direction::VALUES.iter() {
            if self.get_height_from_coordinate(&Coordinate::add_delta_with_direction(
                coordinate, direction,
            )) == height
            {
                directions.push(direction.clone());
            }
        }
        directions
    }
}

#[derive(Debug)]
struct Explorer {
    coordinate: Coordinate,
    map: Map,
    child_explorers: Vec<Self>,
    trails_found: HashSet<Coordinate>,
    standing: bool,
}

enum ExplorerMessages {
    CommitingSuicide,
    ContinuingTrail,
    AddingChildExplorers,
    FinishedExploring,
    Standing,
}

impl Explorer {
    fn new(coordinate: Coordinate, map: Map) -> Self {
        return Self {
            coordinate: coordinate,
            map: map,
            child_explorers: vec![],
            standing: false,
            trails_found: HashSet::new(),
        };
    }

    fn height(&self) -> u32 {
        return self.map.get_height_from_coordinate(&self.coordinate);
    }

    fn update(&mut self) -> ExplorerMessages {
        if self.height() == 9 {
            self.trails_found.insert(self.coordinate);
            return ExplorerMessages::FinishedExploring;
        }
        let mut child_explorers_to_remove = Vec::new();
        for (explorer_index, explorer) in self.child_explorers.iter_mut().enumerate() {
            match explorer.update() {
                ExplorerMessages::ContinuingTrail => {}
                ExplorerMessages::AddingChildExplorers => {}
                ExplorerMessages::CommitingSuicide => {
                    child_explorers_to_remove.push(explorer_index);
                }
                ExplorerMessages::FinishedExploring => {
                    for trail in &explorer.trails_found {
                        self.trails_found.insert(*trail);
                    }
                    child_explorers_to_remove.push(explorer_index);
                }
                ExplorerMessages::Standing => {}
            }
        }
        for &index in child_explorers_to_remove.iter().rev() {
            self.child_explorers.remove(index);
        }

        let directions = self.map.search_around(&self.coordinate, self.height() + 1);
        match directions.len() {
            0 => {
                println!("No directions to move from coordinate: {:?}", self.coordinate);
                return ExplorerMessages::CommitingSuicide;
            }
            1 => {
                self.walk(&directions[0]);
                println!("Walking to coordinate: {:?}", self.coordinate);
                return ExplorerMessages::ContinuingTrail;
            }
            2 | 3 => {
                if !self.standing {
                    for direction in directions.iter() {
                        self.child_explorers.push(Self {
                            coordinate: Coordinate::add_delta_with_direction(
                                &self.coordinate,
                                direction,
                            ),
                            map: self.map.clone(),
                            child_explorers: vec![],
                            trails_found: HashSet::new(),
                            standing: false,
                        });
                    }
                    self.standing = true;
                    println!("Adding child explorers at coordinate: {:?}", self.coordinate);
                    return ExplorerMessages::AddingChildExplorers;
                } else {
                    if self.child_explorers.len() == 0 {
                        println!("Finished exploring at coordinate: {:?}", self.coordinate);
                        return ExplorerMessages::FinishedExploring;
                    }
                    println!("Standing at coordinate: {:?}", self.coordinate);
                    return ExplorerMessages::Standing;
                }
            }
            _ => {
                // Handle other cases
                return ExplorerMessages::CommitingSuicide;
            }
        }
    }
    fn walk(&mut self, direction: &Direction) -> Coordinate {
        self.coordinate = Coordinate::add_delta_with_direction(&self.coordinate, direction);
        self.coordinate
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

#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
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
        return Self {
            row: coordiante.row + delta.0,
            col: coordiante.col + delta.1,
        };
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
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
    const VALUES: [Self; 4] = [Self::Right, Self::Up, Self::Left, Self::Down];
}

pub fn part_one_old(input: &str) -> Option<u32> {
    let map = parse_map(input);
    println!("{:?}", map);
    let possible_trail_heads = find_possible_trail_heads(&map);
    println!("{:?}", possible_trail_heads);
    let mut total_trails: Vec<Coordinate> = vec![];
    for possible_trail_head in possible_trail_heads.iter() {
        let mut explorer = Explorer::new(possible_trail_head.clone(), map.clone());
        loop {
            match explorer.update() {
                ExplorerMessages::ContinuingTrail => {}
                ExplorerMessages::AddingChildExplorers => {}
                ExplorerMessages::CommitingSuicide => {
                    break;
                }
                ExplorerMessages::FinishedExploring => {
                    explorer.trails_found.iter().for_each(|&trail| { total_trails.push(trail);() });
                    break;
                }
                ExplorerMessages::Standing => {}
            }
        }
    }
    println!("{}", total_trails.len());
    Some(total_trails.len()  as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn part_one(input: &str)->Option<u32> {
    let map = parse_input(&input);
    let trailheads = find_trailheads(&map);
    let total_score = trailheads.iter().map(|&th| calculate_score(&map, th)).sum::<usize>();
    Some(total_score as u32)
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

fn find_trailheads(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &height) in row.iter().enumerate() {
            if height == 0 {
                trailheads.push((row_idx, col_idx));
            }
        }
    }
    trailheads
}

fn calculate_score(map: &[Vec<u8>], start: (usize, usize)) -> usize {
    let mut score = 0;
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut stack = vec![start];

    while let Some((row, col)) = stack.pop() {
        if visited[row][col] {
            continue;
        }
        visited[row][col] = true;

        if map[row][col] == 9 {
            score += 1;
            continue;
        }

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for &(dr, dc) in &directions {
            let new_row = row.wrapping_add(dr as usize);
            let new_col = col.wrapping_add(dc as usize);
            if new_row < map.len() && new_col < map[0].len() && !visited[new_row][new_col] {
                if map[new_row][new_col] == map[row][col] + 1 {
                    stack.push((new_row, new_col));
                }
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
