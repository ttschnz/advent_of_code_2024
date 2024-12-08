use std::collections::HashSet;
use std::ops::Range;

type Position = (usize, usize);
type Map = Vec<Position>;
type Dim = usize;
fn get_position(index: usize, dim: usize) -> Position {
    let x = index % dim;
    let y = (index - x) / dim;
    (y, x)
}

#[aoc_generator(day8)]
fn generate_maps(input: &str) -> (Vec<Map>, Dim) {
    let dim = input.find('\n').unwrap();
    let map = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|ch| match ch {
                '.' => None,
                _ => Some(ch as u32),
            })
        })
        .collect::<Vec<Option<u32>>>();

    (
        map.iter()
            .filter_map(|item| *item)
            .collect::<HashSet<u32>>()
            .iter()
            .map(|ascii_index| {
                map.iter()
                    .enumerate()
                    .filter_map(|(index, field)| {
                        if field.is_some_and(|field_value| field_value == *ascii_index) {
                            Some(get_position(index, dim))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Position>>()
            })
            .collect(),
        dim,
    )
}

fn calculate_antinodes(
    (pos_1, pos_2): &(Position, Position),
    bounds: &Range<isize>,
    depth: usize,
    include_towers: bool,
) -> Vec<Position> {
    let delta = (
        pos_2.0 as isize - pos_1.0 as isize,
        pos_2.1 as isize - pos_1.1 as isize,
    );
    let mut antinodes = Vec::new();
    for multiplier in (if include_towers { 0 } else { 1 })..=depth as isize {
        let node = (
            pos_2.0 as isize + delta.0 * multiplier,
            pos_2.1 as isize + delta.1 * multiplier,
        );
        if bounds.contains(&node.0) && bounds.contains(&node.1) {
            antinodes.push((node.0 as usize, node.1 as usize))
        }
        let node = (
            pos_1.0 as isize - delta.0 * multiplier,
            pos_1.1 as isize - delta.1 * multiplier,
        );
        if bounds.contains(&node.0) && bounds.contains(&node.1) {
            antinodes.push((node.0 as usize, node.1 as usize))
        }
    }
    antinodes
}

fn get_antinode_locs(map: &Map, dim: &usize, depth: usize, include_towers: bool) -> Vec<Position> {
    let mut pos_out = Vec::with_capacity(1000);
    let bounds = 0..*dim as isize;
    for pos_idx_1 in 0..map.len() {
        for pos_idx_2 in (pos_idx_1 + 1)..map.len() {
            let pos_1 = map[pos_idx_1];
            let pos_2 = map[pos_idx_2];

            pos_out.append(&mut calculate_antinodes(
                &(pos_1, pos_2),
                &bounds,
                depth,
                include_towers,
            ));
        }
    }
    pos_out
}

#[aoc(day8, part1)]
fn count_unique_antinodes((maps, dim): &(Vec<Map>, Dim)) -> u32 {
    let positions = maps
        .iter()
        .flat_map(|map| get_antinode_locs(map, dim, 1, false))
        .collect::<HashSet<Position>>();

    positions.iter().count() as u32
}
#[aoc(day8, part2)]
fn count_unique_antinodes_depth_2((maps, dim): &(Vec<Map>, Dim)) -> u32 {
    let positions = maps
        .iter()
        .flat_map(|map| get_antinode_locs(map, dim, *dim, true))
        .collect::<HashSet<Position>>();

    positions.iter().count() as u32
}

pub fn part1(input: &str) -> u32 {
    count_unique_antinodes(&generate_maps(input))
}

pub fn part2(input: &str) -> u32 {
    count_unique_antinodes_depth_2(&generate_maps(input))
}

#[cfg(test)]
mod test {
    use super::{count_unique_antinodes, count_unique_antinodes_depth_2, generate_maps};

    const INPUT: &str = r"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    #[test]
    fn test_generator_d8() {
        let (mut maps, _dim) = generate_maps(INPUT);
        maps.sort(); // prevent random shuffling due to maps collect
        assert_eq!(
            maps,
            vec![
                vec![(6, 5), (8, 8), (9, 9)],
                vec![(8, 1), (5, 2), (7, 3), (4, 4)],
            ]
        )
    }

    #[test]
    fn test_count_antinodes() {
        let (maps, dim) = generate_maps(INPUT);
        assert_eq!(count_unique_antinodes(&(maps, dim)), 14);
    }

    #[test]
    fn test_count_antinodes_2() {
        let (maps, dim) = generate_maps(INPUT);
        assert_eq!(count_unique_antinodes_depth_2(&(maps, dim)), 34);
    }
}
