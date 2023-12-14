struct Grid {
    tiles: Vec<Vec<(Tile, bool)>>,
    start: Pos,
}
impl Grid {
    fn new(input: &str) -> Grid {
        let tiles = input.lines()
            .map(|row| row.chars()
                .map(|tile| (Tile::from(tile), false))
                .collect::<Vec<(Tile, bool)>>()
            )
            .collect::<Vec<Vec<(Tile, bool)>>>();

        let mut grid = Grid {
            tiles,
            start: Pos::new(0, 0),
        };

        for row in 0..grid.tiles.len() {
            for col in 0..grid.tiles[row].len() {
                if let Tile::Start = grid.tiles[row][col].0 {
                    grid.start = Pos::new(row, col);
                }
            }
        }

        grid
    }

    fn next_tile(&mut self, pos: &Pos) -> Option<Pos> {
        self.tiles[pos.row][pos.col].1 = true;
        let current = &self.tiles[pos.row][pos.col].0;
        let mut connected = None;

        // move north
        if pos.row > 0  {
            let neighbour = Pos::new(pos.row - 1, pos.col);
            let (tile, visited) = &self.tiles[neighbour.row][neighbour.col];

            match (current, tile) {
                (
                    Tile::Start | Tile::NS | Tile::NE | Tile::NW,
                    Tile::NS | Tile::SE | Tile::SW,
                ) => {
                    if !visited {
                        connected = Some(neighbour);
                    }
                },
                _ => (),
            }
        }

        // move south
        if pos.row < self.tiles.len() - 1 {
            let neighbour = Pos::new(pos.row + 1, pos.col);
            let (tile, visited) = &self.tiles[neighbour.row][neighbour.col];

            match (current, tile) {
                (
                    Tile::Start | Tile::NS | Tile::SE | Tile::SW,
                    Tile::NS | Tile::NE | Tile::NW,
                ) => {
                    if !visited {
                        connected = Some(neighbour);
                    }
                },
                _ => (),
            }
        }

        // move west
        if pos.col > 0 {
            let neighbour = Pos::new(pos.row, pos.col - 1);
            let (tile, visited) = &self.tiles[neighbour.row][neighbour.col];

            match (current, tile) {
                (
                    Tile::Start | Tile::EW | Tile::NW | Tile::SW,
                    Tile::EW | Tile::NE | Tile::SE,
                ) => {
                    if !visited {
                        connected = Some(neighbour);
                    }
                },
                _ => (),
            }
        }

        // move east
        if pos.col < self.tiles[pos.row].len() - 1 {
            let neighbour = Pos::new(pos.row, pos.col + 1);
            let (tile, visited) = &self.tiles[neighbour.row][neighbour.col];

            match (current, tile) {
                (
                    Tile::Start | Tile::EW | Tile::NE | Tile::SE,
                    Tile::EW | Tile::NW | Tile::SW,
                ) => {
                    if !visited {
                        connected = Some(neighbour);
                    }
                },
                _ => (),
            }
        }

        connected
    }

    fn pad(&mut self) {
        for row in self.tiles.iter_mut() {
            row.insert(0, (Tile::Ground, false));
            row.push((Tile::Ground, false));
        }

        self.tiles.insert(0, vec![(Tile::Ground, false); self.tiles[0].len()]);
        self.tiles.push(vec![(Tile::Ground, false); self.tiles[0].len()]);

        self.start.row += 1;
        self.start.col += 1;
    }
}

#[derive(Clone, Debug)]
struct Pos {
    row: usize,
    col: usize,
}
impl Pos {
    fn new(row: usize, col: usize) -> Pos {
        Pos { row, col }
    }
}

#[derive(Clone)]
enum Tile {
    NS,
    EW,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}
impl From<char> for Tile {
    fn from(value: char) -> Tile {
        match value {
            '|' => Tile::NS,
            '-' => Tile::EW,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!(),
        }
    }
}

pub fn solve_part_one(input: &str) -> usize {
    let mut grid = Grid::new(input);

    let mut current = grid.next_tile(&grid.start.clone()).unwrap();
    let mut moves = 1;

    while let Some(next) = grid.next_tile(&current) {
        current = next;
        moves += 1;
    }
    
    (moves + 1) / 2
}

pub fn solve_part_two(input: &str) -> usize {
    let mut grid = Grid::new(input);
    grid.pad();

    let mut current = grid.next_tile(&grid.start.clone()).unwrap();

    while let Some(next) = grid.next_tile(&current) {
        current = next;
    }
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_A: &str = "\
        -L|F7\n\
        7S-7|\n\
        L|7||\n\
        -L-J|\n\
        L|-JF\n\
    ";

    const INPUT_B: &str = "\
        7-F7-\n\
        .FJ|7\n\
        SJLL7\n\
        |F--J\n\
        LJ.LJ\n\
    ";

    const INPUT_C: &str = "\
        ...........\n\
        .S-------7.\n\
        .|F-----7|.\n\
        .||.....||.\n\
        .||.....||.\n\
        .|L-7.F-J|.\n\
        .|..|.|..|.\n\
        .L--J.L--J.\n\
        ...........\n\
    ";

    const INPUT_D: &str = "\
        .F----7F7F7F7F-7....\n\
        .|F--7||||||||FJ....\n\
        .||.FJ||||||||L7....\n\
        FJL7L7LJLJ||LJ.L-7..\n\
        L--J.L7...LJS7F-7L7.\n\
        ....F-J..F7FJ|L7L7L7\n\
        ....L7.F7||L7|.L7L7|\n\
        .....|FJLJ|FJ|F7|.LJ\n\
        ....FJL-7.||.||||...\n\
        ....L---J.LJ.LJLJ...\n\
    ";

    #[test]
    fn part_one_a() {
        let expected = 4;

        assert_eq!(solve_part_one(INPUT_A), expected);
    }

    #[test]
    fn part_one_b() {
        let expected = 8;

        assert_eq!(solve_part_one(INPUT_B), expected);
    }

    #[test]
    fn part_two_c() {
        let expected = 0;

        assert_eq!(solve_part_two(INPUT_C), expected);
    }

    #[test]
    fn part_two_d() {
        let expected = 0;

        assert_eq!(solve_part_two(INPUT_D), expected);
    }
}
