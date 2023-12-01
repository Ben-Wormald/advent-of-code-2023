pub fn solve_part_one(input: &str) -> usize {
    input.lines().into_iter()
        .fold(0, |sum, line| {
            let digits = line.chars().into_iter()
                .filter(|c| c.is_numeric())
                .map(|d| d.to_string().parse().unwrap())
                .collect::<Vec<usize>>();

            let digit_one = digits.first().unwrap();
            let digit_two = digits.last().unwrap();
            
            let value: usize = format!("{digit_one}{digit_two}").parse().unwrap();

            sum + value
        })
}

const WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn solve_part_two(input: &str) -> usize {
    input.lines().into_iter()
        .fold(0, |sum, line| {
            let mut digit_one: Option<usize> = None;
            let mut digit_two: Option<usize> = None;

            for (index, c) in line.char_indices() {
                if let Ok(digit) = c.to_string().parse::<usize>() {
                    digit_one = Some(digit);
                    break;
                }

                if let Some(word) = WORDS.into_iter().find(|word| line[index..].starts_with(word)) {
                    digit_one = Some(word_to_digit(word));
                    break;
                }
            }

            for (index, c) in line.char_indices().rev() {
                if let Ok(digit) = c.to_string().parse::<usize>() {
                    digit_two = Some(digit);
                    break;
                }

                if let Some(word) = WORDS.into_iter().find(|word| line[index..].starts_with(word)) {
                    digit_two = Some(word_to_digit(word));
                    break;
                }
            }

            let value: usize = format!(
                "{}{}",
                digit_one.unwrap(),
                digit_two.unwrap(),
            ).parse().unwrap();

            sum + value
        })
}

fn word_to_digit(word: &str) -> usize {
    match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        1abc2\n\
        pqr3stu8vwx\n\
        a1b2c3d4e5f\n\
        treb7uchet\n\
    ";

    const INPUT_TWO: &str = "\
        two1nine\n\
        eightwothree\n\
        abcone2threexyz\n\
        xtwone3four\n\
        4nineeightseven2\n\
        zoneight234\n\
        7pqrstsixteen\n\
    ";

    #[test]
    fn part_one() {
        let expected = 142;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 281;

        assert_eq!(solve_part_two(INPUT_TWO), expected);
    }
}
