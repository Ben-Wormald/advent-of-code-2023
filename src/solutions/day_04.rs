pub fn solve_part_one(input: &str) -> usize {
    input.lines().map(|line| {
        let (_, card) = line.split_once(": ").unwrap();
        let (winning, have) = card.split_once(" | ").unwrap();
        let winning = winning.split_whitespace().collect::<Vec<&str>>();
        let have = have.split_whitespace().collect::<Vec<&str>>();

        have.into_iter().fold(0, |points, number| {
            if winning.contains(&number) {
                if points == 0 { 1 } else { points * 2 }
            } else {
                points
            }
        })
    }).sum()
}

struct Card {
    matches: usize,
    copies: usize,
}
impl Card {
    fn new(matches: usize) -> Card {
        Card {
            matches,
            copies: 1,
        }
    }
}

pub fn solve_part_two(input: &str) -> usize {
    let mut cards = input.lines().map(|line| {
        let line = line.replace("Card ", "");
        let (_, card) = line.split_once(": ").unwrap();

        let (winning, have) = card.split_once(" | ").unwrap();
        let winning = winning.split_whitespace().collect::<Vec<&str>>();
        let have = have.split_whitespace().collect::<Vec<&str>>();

        let matches = have.into_iter().filter(|number| winning.contains(number)).count();

        Card::new(matches)
    }).collect::<Vec<Card>>();

    for index in 0..cards.len() {
        let current_card = cards.get(index).unwrap();
        let copies = current_card.copies;

        for m in 1..=current_card.matches {
            if let Some(card) = cards.get_mut(index + m) {
                card.copies += copies;
            }
        }
    }

    cards.into_iter().map(|card| card.copies).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n\
    ";

    #[test]
    fn part_one() {
        let expected = 13;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 30;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
