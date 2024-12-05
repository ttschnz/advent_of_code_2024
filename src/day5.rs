use std::cmp::Ordering;

#[aoc(day5, part1, Direct)]
pub fn sum_middle_pages_direct(input: &str) -> u32 {
    sum_middle_pages_fast(&generator(input))
}
#[aoc(day5, part2, Direct)]
pub fn sum_middle_pages_ordered_direct(input: &str) -> u32 {
    sum_middle_pages_ordered_fast(&generator(input))
}

pub use sum_middle_pages_direct as part1;
pub use sum_middle_pages_ordered_direct as part2;

type PageId = u8;
type Update = Vec<PageId>;

#[derive(Debug)]
struct Rule {
    leading: PageId,
    trailing: PageId,
}

fn compare(a: &PageId, b: &PageId, rules: &Vec<Rule>) -> Ordering {
    match rules.iter().find(|rule| {
        (rule.leading == *a && rule.trailing == *b) || (rule.trailing == *a && rule.leading == *b)
    }) {
        Some(Rule {
            leading,
            trailing: _,
        }) => {
            if leading == a {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }

        _ => Ordering::Equal,
    }
}

impl Rule {
    fn satisfied(&self, update: &Update) -> bool {
        let failed = update
            .iter()
            .enumerate()
            .find(|(_, &page)| page == self.trailing)
            .is_some_and(|(trailing_index, _)| {
                update[trailing_index..]
                    .iter()
                    .find(|&&page| page == self.leading)
                    .is_some()
            });
        !failed
    }
}

#[aoc_generator(day5)]
fn generator(input: &str) -> (Vec<Rule>, Vec<Update>) {
    let (_rules_done, rules, updates) = input.lines().into_iter().fold(
        (false, Vec::new(), Vec::new()),
        |(rules_done, mut rules, mut updates), line| {
            if line == "" {
                return (true, rules, updates);
            }
            if rules_done {
                updates.push(
                    line.split(',')
                        .map(|page| page.parse::<PageId>().unwrap())
                        .collect::<Update>(),
                )
            } else {
                let (leading, trailing) = line.split_once('|').unwrap();

                rules.push(Rule {
                    leading: leading.parse::<PageId>().unwrap(),
                    trailing: trailing.parse::<PageId>().unwrap(),
                });
            }
            (rules_done, rules, updates)
        },
    );
    (rules, updates)
}

fn check_rules_for_update(rules: &Vec<Rule>, update: &Update) -> bool {
    rules.iter().all(|rule| rule.satisfied(update))
}

#[aoc(day5, part1, CheckRules)]
fn sum_middle_pages(input: &(Vec<Rule>, Vec<Update>)) -> u32 {
    let (rules, updates) = input;
    updates
        .iter()
        .filter(|update| check_rules_for_update(rules, update))
        .map(|update| update[update.len() / 2] as u32)
        .sum()
}

#[aoc(day5, part1, CheckSorted)]
fn sum_middle_pages_fast(input: &(Vec<Rule>, Vec<Update>)) -> u32 {
    let (rules, updates) = input;
    updates
        .iter()
        .filter(|update| update.is_sorted_by(|a, b| compare(a, b, rules) == Ordering::Less))
        .map(|update| update[update.len() / 2] as u32)
        .sum()
}

#[aoc(day5, part2, FilterByChecking)]
fn sum_middle_pages_ordered(input: &(Vec<Rule>, Vec<Update>)) -> u32 {
    let (rules, updates) = input;

    updates
        .clone()
        .iter_mut()
        .filter(|update| !check_rules_for_update(rules, update))
        .map(|update| {
            update.sort_by(|a, b| compare(a, b, rules));
            update
        })
        .map(|update| update[update.len() / 2] as u32)
        .sum()
}

#[aoc(day5, part2, FilterByUnSorted)]
fn sum_middle_pages_ordered_fast(input: &(Vec<Rule>, Vec<Update>)) -> u32 {
    let (rules, updates) = input;

    updates
        .clone()
        .iter_mut()
        .filter_map(|update| {
            if update.is_sorted_by(|a, b| compare(a, b, rules) == Ordering::Less) {
                None
            } else {
                update.sort_by(|a, b| compare(a, b, rules));
                Some(update)
            }
        })
        .map(|update| update[update.len() / 2] as u32)
        .sum()
}

#[cfg(test)]
mod test {
    use super::{generator, sum_middle_pages, sum_middle_pages_ordered};
    #[test]
    fn test_generator() {
        generator("47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47");
    }
    #[test]
    fn test_count_middle_pages() {
        let gen = generator("47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47");
        assert_eq!(sum_middle_pages(&gen), 143);
    }
    #[test]
    fn test_count_middle_pages_ordered() {
        let gen = generator("47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47");
        assert_eq!(sum_middle_pages_ordered(&gen), 123);
    }
}
