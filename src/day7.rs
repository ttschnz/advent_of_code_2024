use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Equation {
    solution: f64,
    parts: VecDeque<usize>,
}

impl Equation {
    fn default_unsolveable() -> Self {
        Equation {
            solution: 0.0,
            parts: VecDeque::from_iter(1..=1),
        }
    }

    fn solveable(&self, allow_concat: bool) -> bool {
        match self.parts.len() {
            2.. => {
                let (added, multiplied, concat) = self.find_subequations(allow_concat);
                added.solveable(allow_concat)
                    || multiplied.solveable(allow_concat)
                    || (allow_concat && concat.solveable(allow_concat))
            }
            1 => self.solution == self.parts[0] as f64,
            _ => unreachable!("parts should never be empty"),
        }
    }
    fn find_subequations(&self, allow_concat: bool) -> (Equation, Equation, Equation) {
        let mut added = self.clone();
        let a = added.parts.pop_front().unwrap();
        let b = added.parts.pop_front().unwrap();
        added.parts.push_front(a + b);

        let mut multiplied = self.clone();
        let a = multiplied.parts.pop_front().unwrap();
        let b = multiplied.parts.pop_front().unwrap();
        multiplied.parts.push_front(a * b);

        let concat = if allow_concat {
            let mut concat = self.clone();
            let a = concat.parts.pop_front().unwrap();
            let b = concat.parts.pop_front().unwrap();
            concat.parts.push_front(a * 10usize.pow(b.ilog10() + 1) + b);
            concat
        } else {
            Equation::default_unsolveable()
        };

        (added, multiplied, concat)
    }
}

#[aoc_generator(day7)]
fn generate_equations(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (solution, parts) = line.split_once(':').unwrap();
            Equation {
                solution: solution.parse().unwrap(),
                parts: parts
                    .trim()
                    .split(' ')
                    .map(|part| part.parse::<usize>().unwrap())
                    .collect(),
            }
        })
        .collect()
}

#[aoc(day7, part1)]
fn sum_solveable_equations(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter_map(|equation| {
            if equation.solveable(false) {
                Some(equation.solution)
            } else {
                None
            }
        })
        .sum::<f64>() as usize
}

#[aoc(day7, part2)]
fn sum_solveable_equations_concat(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter_map(|equation| {
            if equation.solveable(true) {
                Some(equation.solution)
            } else {
                None
            }
        })
        .sum::<f64>() as usize
}

#[cfg(test)]
mod test {
    use crate::day7::sum_solveable_equations_concat;

    use super::{generate_equations, sum_solveable_equations};

    #[test]
    fn test_generator() {
        let set = generate_equations("190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20");
        println!("{:?}", set);
    }

    #[test]
    fn test_pt1() {
        let set = generate_equations("190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20");
        assert_eq!(sum_solveable_equations(&set), 3749);
    }

    #[test]
    fn test_pt2() {
        let set = generate_equations("190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20");
        assert_eq!(sum_solveable_equations_concat(&set), 11387);
    }
}
