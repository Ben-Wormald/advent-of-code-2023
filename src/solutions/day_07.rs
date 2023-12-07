use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}
impl Hand {
    fn to_value(&self) -> usize {
        let mut counts = HashMap::<char, usize>::new();
        for card in self.cards.iter() {
            *counts.entry(*card).or_default() += 1;
        }

        if counts.iter().any(|(_, count)| *count == 5) {
            6
        } else if counts.iter().any(|(_, count)| *count == 4) {
            5
        } else if counts.iter().any(|(_, count)| *count == 3) {
            if counts.iter().any(|(_, count)| *count == 2) {
                4
            } else {
                3
            }
        } else if counts.iter().filter(|(_, count)| **count == 2).count() == 2 {
            2
        } else if counts.iter().any(|(_, count)| *count == 2) {
            1
        } else {
            0
        }
    }

    fn compare_tie(&self, other: &Hand, index: usize) -> std::cmp::Ordering {
        let self_value = self.cards.get(index).unwrap().to_value();
        let other_value = other.cards.get(index).unwrap().to_value();

        match self_value.cmp(&other_value) {
            std::cmp::Ordering::Equal => {
                if index + 1 < self.cards.len() {
                    self.compare_tie(other, index + 1)
                } else {
                    std::cmp::Ordering::Equal
                }
            },
            ordering => ordering,
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        match self.to_value().cmp(&other.to_value()) {
            std::cmp::Ordering::Equal => self.compare_tie(other, 0),
            ordering => ordering,
        }
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

trait Card {
    fn to_value(&self) -> usize;
    fn to_value_joker(&self) -> usize;
}
impl Card for char {
    fn to_value(&self) -> usize {
        match self {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => self.to_digit(10).unwrap() as usize,
        }
    }

    fn to_value_joker(&self) -> usize {
        match self {
            'J' => 1,
            _ => self.to_value()
        }
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let mut hands = parse(input);
    hands.sort();
    hands.into_iter().enumerate().map(|(rank, hand)| (rank + 1) * hand.bid).sum()
}

fn parse(input: &str) -> Vec<Hand> {
    input.lines().map(|line| {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards.chars().collect();
        let bid = bid.parse().unwrap();
        Hand {
            cards,
            bid,
        }
    }).collect()
}

#[derive(Debug, Eq, PartialEq)]
struct HandTwo {
    cards: Vec<char>,
    bid: usize,
}
impl HandTwo {
    fn to_value(&self) -> usize {
        let mut counts = HashMap::<char, usize>::new();
        for card in self.cards.iter() {
            *counts.entry(*card).or_default() += 1;
        }

        let joker_count = *counts.get(&'J').unwrap_or(&0);

        if let Some((max_card, _)) = counts.iter()
            .filter(|(card, _)| **card != 'J')
            .max_by(|(_, count_a), (_, count_b)| count_a.cmp(count_b))
        {
            let max_card = *max_card;
            *counts.get_mut(&max_card).unwrap() += joker_count;

            if joker_count > 0 {
                *counts.get_mut(&'J').unwrap() = 0;
            }
        }

        if counts.iter().any(|(_, count)| *count == 5) {
            6
        } else if counts.iter().any(|(_, count)| *count == 4) {
            5
        } else if counts.iter().any(|(_, count)| *count == 3) {
            if counts.iter().any(|(_, count)| *count == 2) {
                4
            } else {
                3
            }
        } else if counts.iter().filter(|(_, count)| **count == 2).count() == 2 {
            2
        } else if counts.iter().any(|(_, count)| *count == 2) {
            1
        } else {
            0
        }
    }

    fn compare_tie(&self, other: &HandTwo, index: usize) -> std::cmp::Ordering {
        let self_value = self.cards.get(index).unwrap().to_value_joker();
        let other_value = other.cards.get(index).unwrap().to_value_joker();

        match self_value.cmp(&other_value) {
            std::cmp::Ordering::Equal => {
                if index + 1 < self.cards.len() {
                    self.compare_tie(other, index + 1)
                } else {
                    std::cmp::Ordering::Equal
                }
            },
            ordering => ordering,
        }
    }
}
impl Ord for HandTwo {
    fn cmp(&self, other: &HandTwo) -> std::cmp::Ordering {
        match self.to_value().cmp(&other.to_value()) {
            std::cmp::Ordering::Equal => self.compare_tie(other, 0),
            ordering => ordering,
        }
    }
}
impl PartialOrd for HandTwo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn solve_part_two(input: &str) -> usize {
    let mut hands = parse_two(input);
    hands.sort();
    hands.into_iter().enumerate().map(|(rank, hand)| (rank + 1) * hand.bid).sum()
}

fn parse_two(input: &str) -> Vec<HandTwo> {
    input.lines().map(|line| {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = cards.chars().collect();
        let bid = bid.parse().unwrap();
        HandTwo {
            cards,
            bid,
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483\n\
    ";

    #[test]
    fn part_one() {
        let expected = 6440;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 5905;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
