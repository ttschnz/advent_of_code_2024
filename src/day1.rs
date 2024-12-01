use std::collections::HashMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in input.lines() {
        let mut locations = line.trim().split("   ").map(|d| d.parse::<u32>().unwrap());
        list1.push(locations.next().unwrap());
        list2.push(locations.next().unwrap());
    }

    (list1, list2)
}

#[aoc(day1, part1)]
pub fn get_total_distance(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut list1 = input.0.clone();
    let mut list2 = input.1.clone();

    list1.sort();
    list2.sort();

    list1
        .iter()
        .zip(list2)
        .fold(0, |acc, (a, b)| acc + b.abs_diff(*a))
}

#[aoc(day1, part2)]
pub fn get_similarity_score(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let list1 = &input.0;
    let list2 = &input.1;

    let mut map = HashMap::new();
    for item in list2 {
        map.entry(item).and_modify(|count| *count += 1).or_insert(1);
    }

    list1.iter().fold(0, |acc, item| {
        acc + map.get(item).map(|value| value * item).unwrap_or(0)
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
