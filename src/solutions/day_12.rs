use std::iter;

use itertools::Itertools;

struct Record {
    states: Vec<State>,
    groups: Vec<usize>,
}
impl Record {
    fn new(line: &str) -> Record {
        let (states, groups) = line.split_once(' ').unwrap();

        let states = states.chars().into_iter().map(|c| match c {
            '.' => State::Working,
            '#' => State::Broken,
            '?' => State::Unknown,
            _ => panic!(),
        }).collect();

        let groups = groups.split(',').map(|n| n.parse().unwrap()).collect();

        Record {
            states,
            groups,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum State {
    Working,
    Broken,
    Unknown,
}

pub fn solve_part_one(input: &str) -> usize {
    input.lines().map(|line| {
        let record = Record::new(line);
        let unknowns = record.states.iter().filter(|s| **s == State::Unknown).count();

        itertools::repeat_n(
            [State::Working, State::Broken].into_iter(),
            unknowns,
        ).multi_cartesian_product().filter(|combination| {
            let mut contiguous = 0;
            let mut combination_index = 0;

            let mut groups = Vec::new();

            for state in record.states.iter() {
                let state = match state {
                    State::Unknown => {
                        let state = &combination[combination_index];
                        combination_index += 1;
                        state
                    },
                    state => state,
                };

                match state {
                    State::Working => {
                        if contiguous > 0 {
                            groups.push(contiguous);
                            contiguous = 0;
                        }
                    },
                    State::Broken => {
                        contiguous += 1;
                    },
                    _ => panic!(),
                } 
            }

            if contiguous > 0 {
                groups.push(contiguous);
            }

            groups.len() == record.groups.len() &&
                iter::zip(groups, record.groups.clone()).all(|(a, b)| a == b)
        }).count()
    }).sum()
}

pub fn solve_part_two(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        ???.### 1,1,3\n\
        .??..??...?##. 1,1,3\n\
        ?#?#?#?#?#?#?#? 1,3,1,6\n\
        ????.#...#... 4,1,1\n\
        ????.######..#####. 1,6,5\n\
        ?###???????? 3,2,1\n\
    ";

    #[test]
    fn part_one() {
        let expected = 21;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 525152;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
