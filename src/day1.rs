// use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let parse = |s: &str| s.trim().parse::<u32>().unwrap();
    for line in input.lines() {
        let (left, right) = line.split_once(' ').unwrap();

        list1.push(parse(left));
        list2.push(parse(right));
    }

    (list1, list2)
}

#[aoc(day1, part1)]
pub fn get_total_distance(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut list1 = input.0.clone();
    let mut list2 = input.1.clone();

    list1.sort_unstable();
    list2.sort_unstable();

    list1
        .iter()
        .zip(list2)
        .fold(0, |acc, (a, b)| acc + b.abs_diff(*a))
}

#[aoc(day1, part2)]
pub fn get_similarity_score(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let list1 = &input.0;
    let list2 = &input.1;

    let max_val = list2.iter().max().unwrap_or(&0);

    let mut map: Vec<u32> = vec![0; (max_val + 1) as usize];

    for item in list2 {
        map[*item as usize] += 1;
    }

    list1.iter().fold(0, |acc, item| {
        acc + if item <= max_val {
            map[*item as usize] * item
        } else {
            0
        }
    })
}

#[cfg(test)]
mod test {
    use super::{get_similarity_score, get_total_distance};

    #[test]
    fn sample1() {
        let sample = (vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]);

        assert_eq!(get_total_distance(&sample), 11);
        assert_eq!(get_similarity_score(&sample), 31);
    }
}
