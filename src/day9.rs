use std::{
    collections::VecDeque,
    iter::{repeat, Repeat, Take},
};

const INPUT_LENGTH: usize = 20000;

enum ReadingState {
    File { file_index: usize },
    FreeSpace { last_file_index: usize },
}

impl ReadingState {
    fn switch(&mut self) {
        match self {
            ReadingState::File { file_index } => {
                *self = ReadingState::FreeSpace {
                    last_file_index: *file_index,
                }
            }
            ReadingState::FreeSpace { last_file_index } => {
                *self = ReadingState::File {
                    file_index: *last_file_index + 1,
                }
            }
        }
    }
    fn get_content(&self, length: u32) -> Take<Repeat<Option<usize>>> {
        repeat(match self {
            ReadingState::File { file_index } => Some(*file_index),
            ReadingState::FreeSpace { last_file_index: _ } => None,
        })
        .take(length as usize)
    }
}
#[aoc_generator(day9, part1)]
fn read_memory(input: &str) -> (Vec<Option<usize>>, u32) {
    let mut memory: Vec<Option<usize>> = Vec::with_capacity(INPUT_LENGTH);
    let chars = input.chars();

    let mut total_filled = 0;
    let mut reading_state = ReadingState::File { file_index: 0 };
    for content_length in chars {
        let content_length = content_length.to_digit(10).unwrap();
        memory.extend(reading_state.get_content(content_length));
        total_filled += content_length;
        reading_state.switch();
    }

    (memory, total_filled)
}

fn calculate_checksum(memory: &[usize]) -> usize {
    memory
        .iter()
        .enumerate()
        .fold(0, |acc, (memory_index, file_index)| {
            acc + memory_index * file_index
        })
}

#[aoc(day9, part1)]
fn consolidate_memory((input, total_filled): &(Vec<Option<usize>>, u32)) -> usize {
    let mut memory = VecDeque::from_iter(input.iter());
    let mut consolidated_memory: Vec<usize> = Vec::with_capacity(*total_filled as usize);
    'consolidation: while !memory.is_empty() {
        let new = if let Some(front) = memory.pop_front().unwrap() {
            *front
        } else {
            while !memory.back().is_some_and(|cnt| cnt.is_some()) {
                if memory.pop_back().is_none() {
                    break 'consolidation;
                }
            }
            memory.pop_back().unwrap().unwrap()
        };

        consolidated_memory.push(new);
    }
    calculate_checksum(&consolidated_memory)
}

#[derive(Debug, Clone, Copy)]
struct FreeSpace {
    offset: usize,
    size: u32,
}
#[derive(Debug, Clone, Copy)]
struct File {
    size: u32,
    offset: usize,
    index: usize,
}

#[aoc_generator(day9, part2)]
fn read_memory_to_blocks(input: &str) -> (Vec<File>, Vec<FreeSpace>) {
    let mut files: Vec<File> = Vec::with_capacity(INPUT_LENGTH);
    let mut free_space: Vec<FreeSpace> = Vec::with_capacity(INPUT_LENGTH);

    let chars = input.chars();
    let mut offset = 0;
    let mut reading_state = ReadingState::File { file_index: 0 };
    for content_length in chars {
        let content_length = content_length.to_digit(10).unwrap();
        match reading_state {
            ReadingState::File { file_index } => files.push(File {
                index: file_index,
                size: content_length,
                offset,
            }),
            ReadingState::FreeSpace { last_file_index: _ } => free_space.push(FreeSpace {
                size: content_length,
                offset,
            }),
        };
        offset += content_length as usize;
        reading_state.switch();
    }

    (files, free_space)
}
#[aoc(day9, part2)]
fn consolidate_memory_fit((files, free_spaces): &(Vec<File>, Vec<FreeSpace>)) -> usize {
    let mut free_spaces = free_spaces.clone();
    let mut files = files.clone();
    files.sort_by_key(|file| file.offset);

    while files.iter_mut().rev().fold(false, |did_move, file| {
        if let Some(space) = free_spaces
            .iter_mut()
            .find(|space| space.size >= file.size && space.offset < file.offset)
        {
            file.offset = space.offset;
            space.size -= file.size;
            space.offset += file.size as usize;
            true
        } else {
            did_move
        }
    }) {
        files.sort_by_key(|file| file.offset);
        #[cfg(test)]
        println!(
            "{:?}",
            (0..43)
                .map(|idx| {
                    files
                        .iter()
                        .find(|file| file.offset <= idx && file.size as usize + file.offset > idx)
                        .map(|file| file.index.to_string())
                        .unwrap_or(".".to_string())
                })
                .collect::<Vec<_>>()
                .join("")
        )
    }
    files.iter().fold(0, |acc, file| {
        acc + (file.offset..(file.offset + file.size as usize))
            .map(|idx| idx * file.index)
            .sum::<usize>()
    })
}

pub fn part1(input: &str) -> usize {
    consolidate_memory(&read_memory(input))
}
pub fn part2(input: &str) -> usize {
    consolidate_memory_fit(&read_memory_to_blocks(input))
}

#[cfg(test)]
mod test {
    use super::{consolidate_memory, consolidate_memory_fit, read_memory, read_memory_to_blocks};

    const INPUT: &str = "2333133121414131402";
    #[test]
    fn test_consolidate_memory() {
        assert_eq!(consolidate_memory(&read_memory(INPUT)), 1928);
    }

    #[test]
    fn test_consolidate_memory_fit() {
        assert_eq!(consolidate_memory_fit(&read_memory_to_blocks(INPUT)), 2858);
    }
}
