use rayon::prelude::*;

type Stone = usize;

fn blink(stone: &Stone, depth: u8) -> Vec<Stone> {
    // println!("reached depth: {}", depth);
    if depth == 0 {
        vec![*stone]
    } else {
        // rule 1: 0 => 1
        // rule 2: even number of digits => split in two
        // rule 3: odd number of digits => multiply by 2024
        let new_stones = if *stone == 0 {
            vec![1]
        } else {
            let mut digit_count = 0_u32;
            let mut n = *stone;
            while n > 0 {
                n /= 10;
                digit_count += 1;
            }
            if digit_count % 2 == 0 {
                // even
                let left_stone = stone / (10_usize.pow(digit_count / 2));
                let right_stone = stone - left_stone * 10_usize.pow(digit_count / 2);
                // println!("splitted {stone} into {left_stone} and {right_stone}");
                vec![left_stone, right_stone]
            } else {
                // println!("multiplying {stone} by 2024");
                vec![*stone * 2024]
            }
        };
        // println!("starting next level for {:?}", new_stones);
        new_stones
            .par_iter()
            .flat_map(|stone| blink(stone, depth - 1))
            .collect()
    }
}

#[aoc(day11, part1)]
fn count_stones_after_blink(input: &str) -> u32 {
    input
        .split(" ")
        .flat_map(|stone_str| blink(&stone_str.parse::<Stone>().unwrap(), 25))
        .count() as u32
}
#[aoc(day11, part2)]
fn count_stones_after_blink_many(input: &str) -> u32 {
    input
        .split(" ")
        .par_bridge()
        .flat_map(|stone_str| blink(&stone_str.parse::<Stone>().unwrap(), 36))
        .count() as u32
}

#[cfg(test)]
mod test {
    use super::count_stones_after_blink;

    #[test]
    fn test_part1() {
        assert_eq!(count_stones_after_blink("125 17"), 55312);
    }
}
