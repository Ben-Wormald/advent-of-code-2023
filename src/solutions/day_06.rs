use std::{fmt, iter, str};

pub fn solve_part_one(input: &str) -> usize {
    let mut lines = input.lines();
    let times = parse_line::<usize>(lines.next());
    let distances = parse_line::<usize>(lines.next());

    iter::zip(times, distances).map(|(time, distance)| {
        let (r1, r2) = get_roots(time, distance);
        let (r1, r2) = (r1.floor() as isize + 1, r2.ceil() as isize - 1);
        (r1 - r2).unsigned_abs() + 1
    }).product()
}

fn parse_line<T>(line: Option<&str>) -> Vec<T> where
    T: str::FromStr,
    <T as str::FromStr>::Err: fmt::Debug,
{
    line.unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

fn get_roots(t: usize, d: usize) -> (f64, f64) {
    let (t, d) = (t as f64, d as f64);
    let r1 = (-t + (t.powf(2.0) - 4.0 * d).sqrt()) / -2.0;
    let r2 = (-t - (t.powf(2.0) - 4.0 * d).sqrt()) / -2.0;
    (r1, r2)
}

pub fn solve_part_two(input: &str) -> usize {
    let mut lines = input.lines();
    let time = parse_line::<String>(lines.next()).join("").parse::<usize>().unwrap();
    let distance = parse_line::<String>(lines.next()).join("").parse::<usize>().unwrap();

    let (r1, r2) = get_roots(time, distance);
    let (r1, r2) = (r1.floor() as isize + 1, r2.ceil() as isize - 1);
    (r1 - r2).unsigned_abs() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        Time:      7  15   30\n\
        Distance:  9  40  200\n\
    ";

    #[test]
    fn part_one() {
        let expected = 288;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 71503;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
