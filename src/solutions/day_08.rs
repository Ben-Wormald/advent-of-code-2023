use num::integer::lcm;

struct Map<'a> {
    directions: Vec<Direction>,
    nodes: Vec<Node<'a>>,
}

enum Direction {
    Left,
    Right,
}
impl From<char> for Direction {
    fn from(value: char) -> Direction {
        match value {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Node<'a> {
    id: &'a str,
    left: &'a str,
    right: &'a str,
}
impl Node<'_> {
    fn next(&self, direction: &Direction) -> &str {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let map = parse(input);

    const START: &str = "AAA";
    const END: &str = "ZZZ";

    count_steps(&map, START, |id| id == END)
}

fn parse(input: &str) -> Map {
    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let directions = directions.chars().map(Direction::from).collect();

    let nodes = nodes.lines().map(|line| {
        let (id, next) = line.split_once(" = ").unwrap();
        let (left, right) = &next[1..(next.len() - 1)].split_once(", ").unwrap();

        Node {
            id,
            left,
            right,
        }
    }).collect();

    Map {
        directions,
        nodes,
    }
}

fn count_steps(map: &Map, start: &str, done: fn(&str) -> bool) -> usize {
    let mut node = map.nodes.iter().find(|n| n.id == start).unwrap();
    let mut steps = 0;

    for direction in map.directions.iter().cycle() {
        let next = node.next(direction);
        node = map.nodes.iter().find(|n| n.id == next).unwrap();
        steps += 1;

        if done(node.id) {
            break;
        }
    }
    steps
}

pub fn solve_part_two(input: &str) -> usize {
    let map = parse(input);

    let step_counts = map.nodes.iter()
        .filter(|n| n.id.ends_with('A'))
        .map(|node| count_steps(&map, node.id, |id| id.ends_with('Z')))
        .collect::<Vec<usize>>();

    find_lcm(&step_counts)
}

fn find_lcm(numbers: &[usize]) -> usize {
    if numbers.len() > 1 {
        lcm(*numbers.first().unwrap(), find_lcm(&numbers[1..]))
    } else {
        *numbers.first().unwrap()    
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_ONE_A: &str = "\
        RL\n\
        \n\
        AAA = (BBB, CCC)\n\
        BBB = (DDD, EEE)\n\
        CCC = (ZZZ, GGG)\n\
        DDD = (DDD, DDD)\n\
        EEE = (EEE, EEE)\n\
        GGG = (GGG, GGG)\n\
        ZZZ = (ZZZ, ZZZ)\n\
    ";

    const INPUT_ONE_B: &str = "\
        LLR\n\
        \n\
        AAA = (BBB, BBB)\n\
        BBB = (AAA, ZZZ)\n\
        ZZZ = (ZZZ, ZZZ)\n\
    ";

    const INPUT_TWO: &str = "\
        LR\n\
        \n\
        11A = (11B, XXX)\n\
        11B = (XXX, 11Z)\n\
        11Z = (11B, XXX)\n\
        22A = (22B, XXX)\n\
        22B = (22C, 22C)\n\
        22C = (22Z, 22Z)\n\
        22Z = (22B, 22B)\n\
        XXX = (XXX, XXX)\n\
    ";

    #[test]
    fn part_one_a() {
        let expected = 2;

        assert_eq!(solve_part_one(INPUT_ONE_A), expected);
    }

    #[test]
    fn part_one_b() {
        let expected = 6;

        assert_eq!(solve_part_one(INPUT_ONE_B), expected);
    }

    #[test]
    fn part_two() {
        let expected = 6;

        assert_eq!(solve_part_two(INPUT_TWO), expected);
    }
}
