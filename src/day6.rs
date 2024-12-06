#[cfg(test)]
use std::fmt::Display;

use std::sync::{Arc, RwLock};

use ndarray::Array2;

#[derive(Debug, Copy, Clone)]
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
    Guard { direction: Direction },
    Obstruction,
    Covered,
}

#[cfg(test)]
impl Display for GuardMapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            GuardMapItem::Covered => '❚',
            GuardMapItem::Empty => '.',
            GuardMapItem::Guard { direction } => match direction {
                Direction::North => '^',
                Direction::East => '>',
                Direction::South => 'v',
                Direction::West => '<',
            },
            GuardMapItem::Obstruction => '#',
        };
        write!(f, "{c}")
    }
}

type GuardMap = Array2<GuardMapItem>;

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
                '#' => GuardMapItem::Obstruction,
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

fn walk_map_recursive(guard_map: &mut GuardMap, mut guard_position: (usize, usize)) {
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
        match item {
            GuardMapItem::Covered | GuardMapItem::Empty => {
                *item = GuardMapItem::Guard { direction };
                guard_map[guard_position] = GuardMapItem::Covered;
                guard_position = new_position;
            }
            GuardMapItem::Obstruction => {
                guard_map[guard_position] = GuardMapItem::Guard {
                    direction: direction.rotate_right(),
                }
            }
            _ => unreachable!(),
        }
        walk_map_recursive(guard_map, guard_position);
    }
}

#[aoc(day6, part1, Thingy)]
fn count_distinct_fields((guard_map, guard_position): &(GuardMap, (usize, usize))) -> u32 {
    let mut guard_map = guard_map.clone();
    walk_map_recursive(&mut guard_map, *guard_position);

    guard_map
        .iter()
        .filter(|item| {
            matches!(
                item,
                GuardMapItem::Covered | GuardMapItem::Guard { direction: _ }
            )
        })
        .count() as u32
}

#[cfg(test)]
mod test {
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
}
