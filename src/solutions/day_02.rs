use itertools::Itertools;

mod totals {
    pub const RED: usize = 12;
    pub const GREEN: usize = 13;
    pub const BLUE: usize = 14;
}

struct Game {
    id: usize,
    sets: Vec<Set>,
}
impl Game {
    fn new(line: &str) -> Game {
        let (id, sets) = line.split(": ").collect_tuple().unwrap();

        let id = id.replace("Game ", "").parse::<usize>().unwrap();
    
        let sets = sets.split("; ").map(|set| {
            let mut new_set = Set::default();
    
            for colour in set.split(", ") {
                let (count, colour) = colour.split(' ').collect_tuple().unwrap();
                let count = count.parse::<usize>().unwrap();
    
                match colour {
                    "red" => new_set.red = count,
                    "green" => new_set.green = count,
                    "blue" => new_set.blue = count,
                    _ => panic!(),
                }
            }
            new_set
        }).collect();
    
        Game {
            id,
            sets,
        }
    }

    fn is_possible(&self) -> bool {
        self.sets.iter().all(|set| set.is_possible())
    }

    fn find_min(&self) -> Set {
        let mut max_red: Option<usize> = None;
        let mut max_green: Option<usize> = None;
        let mut max_blue: Option<usize> = None;

        for set in self.sets.iter() {
            if let Some(red) = max_red {
                if set.red > red {
                    max_red = Some(set.red);
                }
            } else {
                max_red = Some(set.red);
            }

            if let Some(green) = max_green {
                if set.green > green {
                    max_green = Some(set.green);
                }
            } else {
                max_green = Some(set.green);
            }

            if let Some(blue) = max_blue {
                if set.blue > blue {
                    max_blue = Some(set.blue);
                }
            } else {
                max_blue = Some(set.blue);
            }
        }

        Set {
            red: max_red.unwrap(),
            green: max_green.unwrap(),
            blue: max_blue.unwrap(),
        }
    }
}

#[derive(Default)]
struct Set {
    red: usize,
    green: usize,
    blue: usize,
}
impl Set {
    fn is_possible(&self) -> bool {
        self.red <= totals::RED
            && self.green <= totals::GREEN
            && self.blue <= totals::BLUE
    }

    fn get_power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

pub fn solve_part_one(input: &str) -> usize {
    input.lines().fold(0, |sum, line| {
        let game = Game::new(line);

        sum + if game.is_possible() { game.id } else { 0 }
    })
}

pub fn solve_part_two(input: &str) -> usize {
    input.lines().fold(0, |sum, line| {
        let game = Game::new(line);

        let min_set = game.find_min();

        sum + min_set.get_power()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n\
    ";

    #[test]
    fn part_one() {
        let expected = 8;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 2286;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
