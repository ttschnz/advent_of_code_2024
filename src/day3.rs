use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub struct Multiplication {
    left: u16,
    right: u16,
}
// the current state indicates what is expected next.
enum MultiplicationGenerator {
    Indicator { index: u8 },                         // mul
    BracketLeft,                                     // (
    FirstNumber { digits: u16 },                     // 12
    SecondNumber { first_number: u16, digits: u16 }, // 24
    Done,
}
impl MultiplicationGenerator {
    fn is_valid(&self, new_char: char) -> bool {
        match self {
            MultiplicationGenerator::Indicator { index } => match index {
                0 => new_char == 'm',
                1 => new_char == 'u',
                2 => new_char == 'l',
                _ => false,
            },
            MultiplicationGenerator::BracketLeft => new_char == '(',
            MultiplicationGenerator::FirstNumber { digits: _ } => {
                '0' <= new_char && new_char <= '9' || new_char == ','
            }
            MultiplicationGenerator::SecondNumber {
                first_number: _,
                digits: _,
            } => '0' <= new_char && new_char <= '9' || new_char == ')',

            _ => false,
        }
    }
    fn advance(&mut self, new_char: char) -> Option<Multiplication> {
        match self {
            MultiplicationGenerator::Indicator { index } => {
                if *index < 2 {
                    *index += 1
                } else {
                    *self = MultiplicationGenerator::BracketLeft
                }
            }
            MultiplicationGenerator::BracketLeft => {
                *self = MultiplicationGenerator::FirstNumber { digits: 0 }
            }
            MultiplicationGenerator::FirstNumber { digits } => {
                if new_char == ',' {
                    *self = MultiplicationGenerator::SecondNumber {
                        first_number: *digits,
                        digits: 0,
                    };
                } else {
                    *digits = *digits * 10 + (new_char as u16 - '0' as u16);
                }
            }
            MultiplicationGenerator::SecondNumber {
                first_number,
                digits,
            } => {
                if new_char == ')' {
                    let mul = Multiplication {
                        left: *first_number,
                        right: *digits,
                    };
                    *self = MultiplicationGenerator::Done;
                    return Some(mul);
                } else {
                    *digits = *digits * 10 + (new_char as u16 - '0' as u16)
                }
            }
            Self::Done => {}
        }
        None
    }
}

pub fn skip_if_disabled(chars: &mut Peekable<Chars<'_>>) {
    if let Some(peeked) = chars.peek() {
        if *peeked == 'd' {
            // could be a "don't". we peek
            let mut disabler = "don't()".chars().peekable();
            while disabler.peek() == chars.peek() {
                if chars.next().is_none() || disabler.next().is_none() {
                    break;
                }
            }
            if disabler.peek().is_none() {
                // we are disabled. search until we have a "do"
                while let Some(peeked) = chars.peek() {
                    if *peeked == 'd' {
                        let mut enabler = "do()".chars().peekable();
                        while enabler.peek() == chars.peek() {
                            if chars.next().is_none() || enabler.next().is_none() {
                                break;
                            }
                        }
                        if enabler.peek().is_none() {
                            break;
                        }
                    } else {
                        chars.next();
                    }
                }
            }
        }
    }
}

pub fn evaluate_muls(input: &str, skip_donts: bool) -> u32 {
    let mut result = 0;
    let mut chars = input.chars().peekable();
    while chars.peek().is_some() {
        if skip_donts {
            skip_if_disabled(&mut chars);
        }
        let mut current_generator = MultiplicationGenerator::Indicator { index: 0 };
        while let Some(new_char) = chars.next() {
            if current_generator.is_valid(new_char) {
                if let Some(new_mul) = current_generator.advance(new_char) {
                    result += new_mul.left as u32 * new_mul.right as u32;
                    break;
                }
            } else {
                break;
            }
        }
    }
    result
}

#[aoc(day3, part1)]
pub fn evaluate_ignore_do_dont(input: &str) -> u32 {
    let mut result = 0;
    let mut chars = input.chars().peekable();
    while chars.peek().is_some() {
        let mut current_generator = MultiplicationGenerator::Indicator { index: 0 };
        while let Some(new_char) = chars.next() {
            if current_generator.is_valid(new_char) {
                if let Some(new_mul) = current_generator.advance(new_char) {
                    result += new_mul.left as u32 * new_mul.right as u32;
                    break;
                }
            } else {
                break;
            }
        }
    }
    result
}

#[aoc(day3, part2)]
pub fn evaluate_do_dont(input: &str) -> u32 {
    let mut result = 0;
    let mut chars = input.chars().peekable();
    while chars.peek().is_some() {
        skip_if_disabled(&mut chars);
        let mut current_generator = MultiplicationGenerator::Indicator { index: 0 };
        while let Some(new_char) = chars.next() {
            if current_generator.is_valid(new_char) {
                if let Some(new_mul) = current_generator.advance(new_char) {
                    result += new_mul.left as u32 * new_mul.right as u32;
                    break;
                }
            } else {
                break;
            }
        }
    }
    result
}

pub use evaluate_do_dont as part2;
pub use evaluate_ignore_do_dont as part1;

#[cfg(test)]
mod test {
    use super::{part1, part2};
    #[test]
    fn test_muls() {
        assert!(
            part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))") == 161
        );
    }
    #[test]
    fn test_do_dont() {
        assert!(
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))")
                == 48
        );
    }
}
