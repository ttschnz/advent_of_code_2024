use ndarray::Array2;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
type Position = (usize, usize);

const DIRECTIONS: &[(isize, isize)] = &[
    // (-1, -1),
    (-1, 0),
    // (-1, 1),
    (0, -1),
    (0, 1),
    // (1, -1),
    (1, 0),
    // (1, 1),
];

fn get_trailhead_score(starting_pos: &Position, map: &Array2<u8>) -> Vec<Position> {
    let starting_value = map[*starting_pos];
    if starting_value == 9 {
        vec![*starting_pos]
    } else {
        let bounds = 0..map.dim().0 as isize;

        DIRECTIONS
            .iter()
            .filter_map(|direction| {
                let target_pos = (
                    starting_pos.0 as isize + direction.0,
                    starting_pos.1 as isize + direction.1,
                );
                if bounds.contains(&target_pos.0) && bounds.contains(&target_pos.1) {
                    Some((target_pos.0 as usize, target_pos.1 as usize))
                } else {
                    None
                }
            })
            .filter(|target_pos| map[*target_pos] == starting_value + 1)
            .map(|new_starting_pos| {
                get_trailhead_score(&new_starting_pos, map)
                    .into_iter()
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }
}

#[aoc(day10, part1)]
pub fn sum_trailheads_score_unique(input: &str) -> u32 {
    sum_trailheads_score(input, true)
}
pub use sum_trailheads_score_unique as part1;

#[aoc(day10, part2)]
pub fn sum_trailheads_score_non_unique(input: &str) -> u32 {
    sum_trailheads_score(input, false)
}
pub use sum_trailheads_score_non_unique as part2;

fn sum_trailheads_score(input: &str, unique_finish: bool) -> u32 {
    let trailhead_pos = Arc::new(RwLock::new(HashSet::<Position>::with_capacity(1000)));
    let dimension = input.find('\n').unwrap();

    let collected = input
        .lines()
        .enumerate()
        .flat_map(|(line_no, line)| {
            let trailhead_pos = &trailhead_pos;
            line.chars().enumerate().map(move |(ch_no, ch)| {
                let digit = ch as u8 - '0' as u8;
                if digit == 0 {
                    trailhead_pos
                        .write()
                        .map(|mut trailhead_pos| trailhead_pos.insert((line_no, ch_no)))
                        .unwrap();
                }
                digit
            })
        })
        .collect::<Vec<u8>>();

    let map = Array2::from_shape_vec((dimension, dimension), collected).unwrap();

    let sum = trailhead_pos
        .read()
        .unwrap()
        .iter()
        .map(|trailhead| {
            if unique_finish {
                get_trailhead_score(trailhead, &map)
                    .into_iter()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect::<Vec<_>>()
            } else {
                get_trailhead_score(trailhead, &map)
                    .into_iter()
                    .collect::<Vec<_>>()
            }
        })
        .flatten()
        .count() as u32;

    sum
}

#[cfg(test)]
mod test {
    use super::{sum_trailheads_score_non_unique, sum_trailheads_score_unique};

    const INPUT: &str = r"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_pt1() {
        assert_eq!(sum_trailheads_score_unique(INPUT), 36);
    }
    #[test]
    fn test_pt2() {
        assert_eq!(sum_trailheads_score_non_unique(INPUT), 81);
    }
}
