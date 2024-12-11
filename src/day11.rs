use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

type Stone = usize;

fn blink(stone: &Stone, depth: u8, lookup: Arc<RwLock<HashMap<(Stone, u8), usize>>>) -> usize {
    // println!("reached depth: {}", depth);
    if depth == 0 {
        1 // there is one stone
    } else {
        if let Some(looked_up_count) = lookup
            .read()
            .ok()
            .and_then(|lookup| lookup.get(&(*stone, depth)).map(|val| val.clone()))
        {
            looked_up_count.clone()
        } else {
            // rule 1: 0 => 1
            // rule 2: even number of digits => split in two
            // rule 3: odd number of digits => multiply by 2024
            let count = if *stone == 0 {
                blink(&1, depth - 1, lookup.clone())
            } else {
                let mut digit_count = 0_u32;
                let mut n = *stone;
                while n > 0 {
                    n /= 10;
                    digit_count += 1;
                }
                if digit_count % 2 == 0 {
                    let left_stone = stone / (10_usize.pow(digit_count / 2));
                    let right_stone = stone - left_stone * 10_usize.pow(digit_count / 2);
                    let left_count = blink(&left_stone, depth - 1, lookup.clone());
                    let right_count = blink(&right_stone, depth - 1, lookup.clone());
                    left_count + right_count
                } else {
                    blink(&(*stone * 2024), depth - 1, lookup.clone())
                }
            };
            lookup
                .write()
                .ok()
                .and_then(|mut writable_lookup| writable_lookup.insert((*stone, depth), count));
            count
        }
    }
}

#[aoc(day11, part1)]
pub fn count_stones_after_blink(input: &str) -> usize {
    let lookup = Arc::new(RwLock::new(HashMap::new()));
    input.split(" ").fold(0, |acc, stone_str| {
        acc + blink(&stone_str.parse::<Stone>().unwrap(), 25, lookup.clone())
    })
}
#[aoc(day11, part2)]
pub fn count_stones_after_blink_many(input: &str) -> usize {
    let lookup = Arc::new(RwLock::new(HashMap::new()));
    input
        .split(" ")
        .map(|stone_str| blink(&stone_str.parse::<Stone>().unwrap(), 75, lookup.clone()))
        .sum()
}

pub use count_stones_after_blink as part1;
pub use count_stones_after_blink_many as part2;

#[cfg(test)]
mod test {
    use super::count_stones_after_blink;

    #[test]
    fn test_part1() {
        assert_eq!(count_stones_after_blink("125 17"), 55312);
    }
}
