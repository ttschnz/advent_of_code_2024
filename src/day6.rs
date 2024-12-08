use ndarray::Array2;
#[cfg(test)]
use std::fmt::Display;
use std::sync::{Arc, RwLock};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn get_vector(&self) -> (isize, isize) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum GuardMapItem {
    Empty,
    Guard {
        direction: Direction,
    },
    Obstruction {
        visited_direction: Option<Direction>,
    },
    Covered,
}

#[cfg(test)]
impl Display for GuardMapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            GuardMapItem::Covered => 'x',
            GuardMapItem::Empty => ' ',
            GuardMapItem::Guard { direction } => match direction {
                Direction::North => '^',
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
            },
            GuardMapItem::Obstruction {
                visited_direction: None,
            } => '#',
            GuardMapItem::Obstruction {
                visited_direction: Some(_),
            } => '%',
        };
        write!(f, "{c}")
    }
}

type GuardMap = Array2<GuardMapItem>;

#[aoc(day6, part1, Direct)]
pub fn count_distinct_fields_direct(input: &str) -> u32 {
    let (mut map, pos) = generate_map(input);
    match walk_map_recursive(&mut map, pos) {
        MapType::Exitable(result) => result,
        _ => panic!("given map for part 1 is a loop"),
    }
}

#[aoc(day6, part2, Direct)]
pub fn count_obstruction_options_direct(input: &str) -> u32 {
    let (initial_guard_map, guard_position) = generate_map(input);
    let mut guard_map = initial_guard_map.clone();
    walk_map_recursive(&mut guard_map, guard_position);
    guard_map
        .indexed_iter()
        .filter_map(|(index, field)| {
            if matches!(field, GuardMapItem::Covered) && index != guard_position {
                Some(index)
            } else {
                None
            }
        })
        .filter(|index| {
            let mut modified = initial_guard_map.clone();
            modified[*index] = GuardMapItem::Obstruction {
                visited_direction: None,
            };
            matches!(
                walk_map_recursive(&mut modified, guard_position),
                MapType::Loop
            )
        })
        .count() as u32
}

pub use count_distinct_fields_direct as part1;
pub use count_obstruction_options_direct as part2;

#[aoc_generator(day6)]
fn generate_map(input: &str) -> (GuardMap, (usize, usize)) {
    let guard_position = Arc::new(RwLock::new((0, 0)));
    let dimension = Arc::new(RwLock::new((0, 0)));

    let lines = input.lines().enumerate().flat_map(|(line_no, line)| {
        let guard_position = &guard_position;
        if line_no == 0 {
            dimension
                .write()
                .map(|mut dim| {
                    *dim = (line.len(), line.len());
                })
                .unwrap()
        }
        line.chars().enumerate().map(move |(char_no, char)| {
            let item = match char {
                '.' => GuardMapItem::Empty,
                '#' => GuardMapItem::Obstruction {
                    visited_direction: None,
                },
                '^' => GuardMapItem::Guard {
                    direction: Direction::North,
                },
                '>' => GuardMapItem::Guard {
                    direction: Direction::East,
                },
                '<' => GuardMapItem::Guard {
                    direction: Direction::West,
                },
                'v' => GuardMapItem::Guard {
                    direction: Direction::South,
                },
                _ => panic!(),
            };

            if matches!(item, GuardMapItem::Guard { direction: _ }) {
                guard_position
                    .write()
                    .map(|mut position| {
                        *position = (line_no, char_no);
                    })
                    .unwrap()
            }
            item
        })
    });

    let collected = lines.collect();
    let map = Array2::from_shape_vec(*dimension.read().unwrap(), collected).unwrap();
    let pos = *guard_position.read().unwrap();
    (map, pos)
}

enum MapType {
    Loop,
    Exitable(u32),
}

fn walk_map_recursive(guard_map: &mut GuardMap, mut guard_position: (usize, usize)) -> MapType {
    let direction = match guard_map[guard_position] {
        GuardMapItem::Guard { direction } => direction,
        _ => unreachable!(),
    };

    let direction_vector = direction.get_vector();
    let new_position = (
        (guard_position.0 as isize + direction_vector.0) as usize,
        (guard_position.1 as isize + direction_vector.1) as usize,
    );
    if let Some(item) = guard_map.get_mut(new_position) {
        let newly_covered = match item {
            GuardMapItem::Covered => {
                guard_map[new_position] = GuardMapItem::Guard { direction };
                guard_map[guard_position] = GuardMapItem::Covered;
                guard_position = new_position;
                0
            }
            GuardMapItem::Empty => {
                guard_map[new_position] = GuardMapItem::Guard { direction };
                guard_map[guard_position] = GuardMapItem::Covered;
                guard_position = new_position;
                1
            }
            GuardMapItem::Obstruction { visited_direction } => {
                if visited_direction.is_some_and(|visited_direction| visited_direction == direction)
                {
                    return MapType::Loop;
                }
                *visited_direction = Some(direction);
                guard_map[guard_position] = GuardMapItem::Guard {
                    direction: direction.rotate_right(),
                };
                0
            }
            _ => unreachable!(),
        };

        match walk_map_recursive(guard_map, guard_position) {
            MapType::Exitable(fields) => MapType::Exitable(fields + newly_covered),
            MapType::Loop => MapType::Loop,
        }
    } else {
        guard_map[guard_position] = GuardMapItem::Covered;
        MapType::Exitable(1)
    }
}

#[aoc(day6, part1)]
fn count_distinct_fields((guard_map, guard_position): &(GuardMap, (usize, usize))) -> u32 {
    let mut guard_map = guard_map.clone();
    match walk_map_recursive(&mut guard_map, *guard_position) {
        MapType::Exitable(result) => result,
        _ => panic!("given map for part 1 is a loop"),
    }
}

#[aoc(day6, part2)]
fn count_obstruction_options((guard_map, guard_position): &(GuardMap, (usize, usize))) -> u32 {
    let mut guard_map = guard_map.clone();
    let initial_guard_map = guard_map.clone();
    walk_map_recursive(&mut guard_map, *guard_position);
    guard_map
        .indexed_iter()
        .filter_map(|(index, field)| {
            if matches!(field, GuardMapItem::Covered) && index != *guard_position {
                Some(index)
            } else {
                None
            }
        })
        .filter(|index| {
            let mut modified = initial_guard_map.clone();
            modified[*index] = GuardMapItem::Obstruction {
                visited_direction: None,
            };
            matches!(
                walk_map_recursive(&mut modified, *guard_position),
                MapType::Loop
            )
        })
        .count() as u32
}

#[cfg(test)]
mod test {
    use crate::day6::count_obstruction_options;

    use super::{count_distinct_fields, generate_map, walk_map_recursive};

    #[test]
    fn test_walk() {
        let (mut map, pos) =generate_map("....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...");
        walk_map_recursive(&mut map, pos);
    }

    #[test]
    fn test_count_distinct_fields() {
        let (map, pos) =generate_map("....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...");
        assert_eq!(count_distinct_fields(&(map, pos)), 41);
    }

    #[test]
    fn test_count_obstruction_options() {
        let (map, pos) =generate_map("....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...");
        assert_eq!(count_obstruction_options(&(map, pos)), 6);
    }
}
