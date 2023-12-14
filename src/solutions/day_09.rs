pub fn solve_part_one(input: &str) -> isize {
    input.lines().map(|sequence| {
        let sequence = sequence
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<isize>>();
        
        get_next(&sequence)
    }).sum()
}

fn get_next(sequence: &[isize]) -> isize {
    let diffs = sequence.windows(2).map(|pair| pair[1] - pair[0]).collect::<Vec<isize>>();

    if diffs.iter().all(|diff| *diff == 0) {
        *sequence.last().unwrap()
    } else {
        sequence.last().unwrap() + get_next(&diffs)
    }
}

pub fn solve_part_two(input: &str) -> isize {
    input.lines().map(|sequence| {
        let mut sequence = sequence
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect::<Vec<isize>>();

        sequence.reverse();
        
        get_next(&sequence)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        0 3 6 9 12 15\n\
        1 3 6 10 15 21\n\
        10 13 16 21 30 45\n\
    ";

    #[test]
    fn part_one() {
        let expected = 114;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 2;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
