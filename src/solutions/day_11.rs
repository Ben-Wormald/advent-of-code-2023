use itertools::Itertools;
use std::{collections::HashSet, cmp};

struct Image {
    galaxies: Vec<Pos>,
    populated_x: HashSet<isize>,
    populated_y: HashSet<isize>,
}
impl Image {
    fn new(input: &str) -> Image {
        let mut galaxies = Vec::new();
        let mut populated_x = HashSet::new();
        let mut populated_y = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    galaxies.push(Pos::new(x, y));
                    populated_x.insert(x as isize);
                    populated_y.insert(y as isize);
                }
            }
        }

        Image {
            galaxies,
            populated_x,
            populated_y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pos {
    x: isize,
    y: isize,
}
impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos {
            x: x as isize,
            y: y as isize,
        }
    }

    fn distance(&self, other: &Pos) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

pub fn solve_part_one(input: &str) -> isize {
    let image = Image::new(input);

    image.galaxies.into_iter().combinations(2).map(|pair| {
        let (a, b) = (pair[0], pair[1]);
        let distance = a.distance(&b);

        let x_min = cmp::min(a.x, b.x);
        let x_max = cmp::max(a.x, b.x);
        let y_min = cmp::min(a.y, b.y);
        let y_max = cmp::max(a.y, b.y);

        let populated_x = image.populated_x.iter().filter(|&&x| x > x_min && x < x_max).count();
        let populated_y = image.populated_y.iter().filter(|&&y| y > y_min && y < y_max).count();
        
        let unpopulated_x = cmp::max(x_max - x_min - 1 - populated_x as isize, 0);
        let unpopulated_y = cmp::max(y_max - y_min - 1 - populated_y as isize, 0);

        distance + unpopulated_x + unpopulated_y
    }).sum()
}

pub fn solve_part_two(input: &str) -> isize {
    let image = Image::new(input);

    // const TIME: isize = 10;
    // const TIME: isize = 100;
    const TIME: isize = 1_000_000;

    image.galaxies.into_iter().combinations(2).map(|pair| {
        let (a, b) = (pair[0], pair[1]);
        let distance = a.distance(&b);

        let x_min = cmp::min(a.x, b.x);
        let x_max = cmp::max(a.x, b.x);
        let y_min = cmp::min(a.y, b.y);
        let y_max = cmp::max(a.y, b.y);

        let populated_x = image.populated_x.iter().filter(|&&x| x > x_min && x < x_max).count();
        let populated_y = image.populated_y.iter().filter(|&&y| y > y_min && y < y_max).count();
        
        let unpopulated_x = cmp::max(x_max - x_min - 1 - populated_x as isize, 0) * (TIME - 1);
        let unpopulated_y = cmp::max(y_max - y_min - 1 - populated_y as isize, 0) * (TIME - 1);

        distance + unpopulated_x + unpopulated_y
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....\n\
    ";

    #[test]
    fn part_one() {
        let expected = 374;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two_a() {
        let expected = 1030;

        assert_eq!(solve_part_two(INPUT), expected);
    }

    #[test]
    fn part_two_b() {
        let expected = 8410;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
