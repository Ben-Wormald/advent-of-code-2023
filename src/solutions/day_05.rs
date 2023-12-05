use itertools::Itertools;

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}
impl Almanac {
    fn new(input: &str) -> Almanac {
        let chunks = input.split("\n\n").collect::<Vec<&str>>();
        let (seeds, maps) = chunks.split_at(1);

        let seeds = seeds
            .first().unwrap()
            .replace("seeds: ", "")
            .split_whitespace()
            .map(|seed| seed.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let maps = maps.into_iter().map(|map| {
            let lines = map.lines().collect::<Vec<&str>>();

            let (types, ranges) = lines.split_at(1);

            let types = types.first().unwrap().replace(" map:", "");
            let (source, destination) = types.split_once("-to-").unwrap();
            let (source, destination) = (source.to_string(), destination.to_string());

            let ranges = ranges.into_iter()
                .map(|range| range
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_tuple().unwrap()
                )
                .collect::<Vec<(usize, usize, usize)>>();

            Map {
                source,
                destination,
                ranges,
            }
        }).collect::<Vec<Map>>();

        Almanac {
            seeds,
            maps,
        }
    }

    fn follow(&self, source: &str, destination: &str, value: usize) -> usize {
        let map = self.maps.iter().find(|map| map.source == source).unwrap();

        let value = map.map(value);

        if map.destination == destination {
            value
        } else {
            self.follow(&map.destination, destination, value)
        }
    }
}

#[derive(Debug)]
struct Map {
    source: String,
    destination: String,
    ranges: Vec<(usize, usize, usize)>,
}
impl Map {
    fn map(&self, value: usize) -> usize {
        let range = self.ranges.iter()
            .find(|(_dest, src, len)|
                value >= *src && value < src + len
            );

        match range {
            Some((dest, src, _len)) => dest + (value - src),
            None => value,
        }
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let almanac = Almanac::new(input);
    
    almanac.seeds.iter().map(|seed| almanac.follow("seed", "location", *seed)).min().unwrap()
}

pub fn solve_part_two(input: &str) -> usize {
    let almanac = Almanac::new(input);

    almanac.seeds
        .chunks(2)
        .flat_map(|pair| {
            let start = pair.get(0).unwrap();
            let len = pair.get(1).unwrap();
            let range = *start..(start + len);

            range.map(|seed| almanac.follow("seed", "location", seed))
        })
        .min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
        seeds: 79 14 55 13\n\
        \n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
        \n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
        \n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
        \n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
        \n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
        \n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
        \n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4\n\
    ";

    #[test]
    fn part_one() {
        let expected = 35;

        assert_eq!(solve_part_one(INPUT), expected);
    }

    #[test]
    fn part_two() {
        let expected = 46;

        assert_eq!(solve_part_two(INPUT), expected);
    }
}
