use std::cmp::Ordering;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split(' ')
                .into_iter()
                .map(|val| val.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

#[aoc(day2, part1, Naïve)]
pub fn count_safe_reports(input: &Vec<Vec<u32>>) -> u32 {
    input
        .into_iter()
        .filter(|report| {
            let mut report = report.into_iter().peekable();
            let mut is_safe = true;
            let mut direction = None;

            while is_safe {
                if let (Some(current), Some(next)) = (report.next(), report.peek()) {
                    if current.abs_diff(**next) > 3 {
                        is_safe = false;
                        break;
                    }
                    let last_direction = direction;
                    match current.cmp(*next) {
                        Ordering::Equal => {
                            is_safe = false;
                            break;
                        }
                        Ordering::Greater => direction = Some(Direction::Decreasing),
                        Ordering::Less => direction = Some(Direction::Increasing),
                    }
                    if last_direction
                        .is_some_and(|last_direction| Some(last_direction) != direction)
                    {
                        is_safe = false;
                        break;
                    }
                } else {
                    break;
                }
            }
            is_safe
        })
        .count() as u32
}

#[aoc(day2, part1, Delta)]
pub fn count_safe_reports_delta(input: &Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        .filter(|report| {
            let mut last_delta_signum = None;
            for data in report.windows(2) {
                let new_delta = data[1] as i64 - data[0] as i64;
                let new_delta_signum = new_delta.signum();

                if last_delta_signum
                    .is_some_and(|last_delta_signum| last_delta_signum != new_delta_signum)
                {
                    return false;
                }

                if !(1..=3).contains(&new_delta.abs()) {
                    return false;
                }

                last_delta_signum = Some(new_delta_signum);
            }
            return true;
        })
        .count() as u32
}

#[aoc(day2, part1, Iterator)]
pub fn count_safe_reports_iterator(input: &Vec<Vec<u32>>) -> u32 {
    input
        .iter()
        .filter(|report| {
            report
                .windows(2)
                .map(|data| data[1] as i64 - data[0] as i64)
                .try_fold(None, |last_signum, curr_delta| {
                    let curr_signum = curr_delta.signum();
                    if last_signum.is_some_and(|last_signum| last_signum != curr_signum) {
                        return Err(());
                    }

                    if !(1..=3).contains(&curr_delta.abs()) {
                        return Err(());
                    }
                    Ok(Some(curr_signum))
                })
                .is_ok()
        })
        .count() as u32
}

#[aoc(day2, part2)]
pub fn count_safe_reports_damped(input: &Vec<Vec<u32>>) -> u32 {
    input
        .into_iter()
        .filter(|report| {
            if count_safe_reports_delta(&vec![(**report).clone()]) == 1 {
                return true;
            } else {
                let single_report = report.iter();
                for n in 0..report.len() {
                    // create an iterator without the nth element
                    if count_safe_reports_delta(&vec![single_report
                        .clone()
                        .enumerate()
                        .filter(move |(i, _)| *i != n)
                        .map(|(_, x)| *x)
                        .collect::<Vec<_>>()])
                        == 1
                    {
                        return true;
                    }
                }
                false
            }
        })
        .count() as u32
}

#[cfg(test)]
mod test {

    use super::{
        count_safe_reports, count_safe_reports_damped, count_safe_reports_delta,
        count_safe_reports_iterator,
    };

    #[test]
    fn sample1() {
        let data = vec![
            (vec![7, 6, 4, 2, 1], true, true),
            (vec![1, 2, 7, 8, 9], false, false),
            (vec![9, 7, 6, 2, 1], false, false),
            (vec![1, 3, 2, 4, 5], false, true),
            (vec![8, 6, 4, 4, 1], false, true),
            (vec![1, 3, 6, 7, 9], true, true),
        ];

        for (sample, expected_normal, expected_damped) in data {
            println!("testing {:?}", sample);
            assert_eq!(
                count_safe_reports(&vec![sample.clone()]),
                if expected_normal { 1 } else { 0 },
                "normal test for {:?} failed.",
                sample
            );
            assert_eq!(
                count_safe_reports_delta(&vec![sample.clone()]),
                if expected_normal { 1 } else { 0 },
                "normal delta test for {:?} failed.",
                sample
            );
            assert_eq!(
                count_safe_reports_iterator(&vec![sample.clone()]),
                if expected_normal { 1 } else { 0 },
                "normal delta test for {:?} failed.",
                sample
            );

            assert_eq!(
                count_safe_reports_damped(&vec![sample.clone()]),
                if expected_damped { 1 } else { 0 },
                "damped test for {:?} failed.",
                sample
            );
        }
    }
}
