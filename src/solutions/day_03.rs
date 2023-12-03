use std::{cmp, collections::{HashMap, HashSet}};

#[derive(Debug)]
enum Cell {
    Digit(char),
    Symbol,
    Gear,
    Empty,
}

trait Grid {
    fn is_adjacent(&self, x: usize, y: usize) -> bool;
    fn adjacent_gears(&self, x: usize, y: usize) -> Vec<(usize, usize)>;
}
impl Grid for Vec<Vec<Cell>> {
    fn is_adjacent(&self, x: usize, y: usize) -> bool {
        let (x, y) = (x as isize, y as isize);

        let mut found = false;

        for n_x in cmp::max(0, x - 1)..cmp::min(self.len() as isize, x + 2) {
            for n_y in cmp::max(0, y - 1)..cmp::min(self.first().unwrap().len() as isize, y + 2) {
                if !(n_x == x && n_y == y) {
                    match self.get(n_x as usize).unwrap().get(n_y as usize).unwrap() {
                        Cell::Symbol | Cell::Gear => found = true,
                        _ => (),
                    }
                }
            }
        }

        found
    }

    fn adjacent_gears(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let (x, y) = (x as isize, y as isize);

        let mut adjacents = Vec::new();

        for n_x in cmp::max(0, x - 1)..cmp::min(self.len() as isize, x + 2) {
            for n_y in cmp::max(0, y - 1)..cmp::min(self.first().unwrap().len() as isize, y + 2) {
                if !(n_x == x && n_y == y) {
                    if let Cell::Gear = self.get(n_x as usize).unwrap().get(n_y as usize).unwrap() {
                        adjacents.push((n_x as usize, n_y as usize));
                    }
                }
            }
        }

        adjacents
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let grid = parse(input);

    let mut sum = 0;

    for (x, row) in grid.iter().enumerate() {
        let mut number = String::new();
        let mut is_adjacent = false;

        for (y, cell) in row.iter().enumerate() {
            match cell {
                Cell::Digit(digit) => {
                    number.push(*digit);

                    if grid.is_adjacent(x, y) {
                        is_adjacent = true;
                    }
                },
                _ => {
                    if is_adjacent && !number.is_empty() {
                        sum += number.parse::<usize>().unwrap();
                    }
                    number = String::new();
                    is_adjacent = false;
                }
            }
        }

        if is_adjacent && !number.is_empty() {
            sum += number.parse::<usize>().unwrap();
        }
    }

    sum
}

fn parse(input: &str) -> Vec<Vec<Cell>> {
    input.lines().map(|line| {
        line.chars().map(|c| {
            match (c.is_numeric(), c) {
                (true, _) => Cell::Digit(c),
                (false, '.') => Cell::Empty,
                (false, '*') => Cell::Gear,
                (false, _) => Cell::Symbol,
            }
        }).collect()
    }).collect()
}

pub fn solve_part_two(input: &str) -> usize {
    let grid = parse(input);

    let mut gears = HashMap::<(usize, usize), HashSet<usize>>::new();
    let mut numbers = HashMap::<usize, usize>::new();

    let mut number = String::new();
    let mut number_id = 0;

    for (x, row) in grid.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            match cell {
                Cell::Digit(digit) => {
                    number.push(*digit);

                    for gear in grid.adjacent_gears(x, y).into_iter() {
                        let adjacent_numbers = gears.entry(gear).or_default();
                        adjacent_numbers.insert(number_id);
                    }
                },
                _ => {
                    if !number.is_empty() {
                        numbers.insert(number_id, number.parse().unwrap());
                        number = String::new();
                        number_id += 1;
                    }
                }
            }
        }

        if !number.is_empty() {
            numbers.insert(number_id, number.parse().unwrap());
        }
        number = String::new();
        number_id += 1;
    }

    gears.into_values().map(|adjacent_numbers| {
        if adjacent_numbers.len() == 2 {
            adjacent_numbers.into_iter()
                .map(|number_id| numbers.get(&number_id).unwrap())
                .product()
        } else {
            0
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..\n\
    ";

    #[test]
    fn part_one() {
        let expected = 4361;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 467835;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
