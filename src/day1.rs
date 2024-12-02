#[aoc(day1, part1, Direct)]
pub fn get_total_distance_direct(input: &str) -> u32 {
    let (mut list_left, mut list_right) = input_generator(input);

    list_left.sort_unstable();
    list_right.sort_unstable();

    list_left
        .into_iter()
        .zip(list_right)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[aoc(day1, part2, Direct)]
pub fn get_similarity_score_direct(input: &str) -> u32 {
    let generated = input_generator(input);
    get_similarity_score(&generated)
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list_left = Vec::new();
    let mut list_right = Vec::new();

    let parse = |s: &str| s.trim().parse::<u32>().unwrap();
    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();

        list_left.push(parse(left));
        list_right.push(parse(right));
    }

    (list_left, list_right)
}

#[aoc(day1, part1, Parsed)]
pub fn get_total_distance_parsed(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut list_left = input.0.clone();
    let mut list_right = input.1.clone();

    list_left.sort_unstable();
    list_right.sort_unstable();

    list_left
        .into_iter()
        .zip(list_right)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

#[aoc(day1, part2)]
pub fn get_similarity_score(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let list_left = &input.0;
    let list_right = &input.1;

    let max_val = list_right.iter().max().unwrap_or(&0);

    let mut map: Vec<u32> = vec![0; (max_val + 1) as usize];

    for item in list_right {
        map[*item as usize] += 1;
    }

    list_left.iter().fold(0, |acc, item| {
        acc + if item <= max_val {
            map[*item as usize] * item
        } else {
            0
        }
    })
}

pub use get_similarity_score_direct as part2;
pub use get_total_distance_direct as part1;

#[cfg(test)]
mod test {
    use super::{get_similarity_score, get_total_distance_parsed};

    #[test]
    fn sample1() {
        let sample = (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);

        assert_eq!(get_total_distance_parsed(&sample), 11);
        assert_eq!(get_similarity_score(&sample), 31);
    }
}
