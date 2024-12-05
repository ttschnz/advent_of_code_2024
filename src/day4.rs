use ndarray::Array2;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum XMasChar {
    X,
    M,
    A,
    S,
}

#[aoc_generator(day4)]
fn generate_data(input: &str) -> Array2<XMasChar> {
    let column_count = input.find('\n').unwrap();
    let flattened = input
        .lines()
        .map(|line| {
            line.chars().map(|char| match char {
                'X' => XMasChar::X,
                'M' => XMasChar::M,
                'A' => XMasChar::A,
                'S' => XMasChar::S,
                _ => panic!("unexpected character"),
            })
        })
        .flatten()
        .collect::<Vec<_>>();

    let line_count = flattened.len() / column_count;
    Array2::from_shape_vec((line_count, column_count), flattened).unwrap()
}

static DIRECTION_UP: u8 = 1 << 0;
static DIRECTION_RIGHT: u8 = 1 << 1;
static DIRECTION_DOWN: u8 = 1 << 2;
static DIRECTION_LEFT: u8 = 1 << 3;

fn get_xmas_directions(index: &(usize, usize), dim: &(usize, usize)) -> Vec<(isize, isize)> {
    let mut directions_allowed = 0;
    if index.0 >= 3 {
        directions_allowed |= DIRECTION_UP
    }
    if index.0 < dim.0 - 3 {
        directions_allowed |= DIRECTION_DOWN
    }
    if index.1 >= 3 {
        directions_allowed |= DIRECTION_LEFT
    }
    if index.1 < dim.1 - 3 {
        directions_allowed |= DIRECTION_RIGHT
    }

    let mut directions = Vec::new();
    if directions_allowed & DIRECTION_UP != 0 && directions_allowed & DIRECTION_LEFT != 0 {
        directions.push((-1, -1));
    }
    if directions_allowed & DIRECTION_UP != 0 && directions_allowed & DIRECTION_RIGHT != 0 {
        directions.push((-1, 1));
    }
    if directions_allowed & DIRECTION_DOWN != 0 && directions_allowed & DIRECTION_LEFT != 0 {
        directions.push((1, -1));
    }
    if directions_allowed & DIRECTION_DOWN != 0 && directions_allowed & DIRECTION_RIGHT != 0 {
        directions.push((1, 1));
    }

    if directions_allowed & DIRECTION_UP != 0 {
        directions.push((-1, 0))
    }
    if directions_allowed & DIRECTION_LEFT != 0 {
        directions.push((0, -1))
    }
    if directions_allowed & DIRECTION_RIGHT != 0 {
        directions.push((0, 1))
    }
    if directions_allowed & DIRECTION_DOWN != 0 {
        directions.push((1, 0))
    }
    directions
}

static XMAS_ORDER: [XMasChar; 4] = [XMasChar::X, XMasChar::M, XMasChar::A, XMasChar::S];

#[aoc(day4, part1)]
fn count_xmas(input: &Array2<XMasChar>) -> u32 {
    let dim = input.dim();
    input
        .indexed_iter()
        .filter(|(_, &ref value)| *value == XMasChar::X)
        .map(|(index, _)| {
            let match_count = get_xmas_directions(&index, &dim)
                .iter()
                .filter(|direction| {
                    (1..=3).all(|fact| {
                        input[(
                            (index.0 as isize + direction.0 * fact as isize) as usize,
                            (index.1 as isize + direction.1 * fact as isize) as usize,
                        )] == XMAS_ORDER[fact]
                    })
                })
                .count() as u32;
            match_count
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::{count_xmas, generate_data};
    #[test]
    fn generator() {
        println!(
            "{:?}",
            generate_data(
                "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX"
            )
        );
    }
    #[test]
    fn part1() {
        assert_eq!(
            count_xmas(&generate_data("MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX")),
            18
        )
    }
}
