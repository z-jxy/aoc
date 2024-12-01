// --- Day 10: Pipe Maze ---
use std::time::Instant;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Direction {
    #[default]
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    SouthNorth,
    EastWest,
}

type Coordinates = [i32; 2];

#[derive(Default, Clone, Copy)]
struct Tile {
    direction: Direction,
    gates: [Coordinates; 2],
    is_edge: bool,
    is_loop: bool,
    is_start: bool,
    position: Coordinates,
}

impl Tile {
    fn new(gates: [Coordinates; 2], position: Coordinates, direction: Direction) -> Tile {
        Tile {
            direction,
            gates,
            is_edge: false,
            is_loop: false,
            is_start: false,
            position,
        }
    }

    fn start_tile(position: Coordinates) -> Tile {
        Tile {
            direction: Direction::default(),
            gates: [[0; 2], [0; 2]],
            is_edge: false,
            is_loop: true,
            is_start: true,
            position,
        }
    }

    fn to(&self, from: Coordinates) -> Coordinates {
        if self.gates[0] == from {
            self.gates[1]
        } else {
            self.gates[0]
        }
    }
}

#[derive(Default)]
struct Maze {
    start: Tile,
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn find_start_gates(&mut self) {
        // Define the ranges around the start tile
        let x_range = [
            (self.start.position[0] - 1).max(0),
            (self.start.position[0] + 1).min(self.tiles[0].len() as i32 - 1),
        ];
        let y_range = [
            (self.start.position[1] - 1).max(0),
            (self.start.position[1] + 1).min(self.tiles.len() as i32 - 1),
        ];

        let mut entries: Vec<Coordinates> = Vec::new();

        // Enumerate over every tile in the tiles Vec that is within our defined range
        for i in y_range[0]..=y_range[1] {
            for j in x_range[0]..=x_range[1] {
                // Skip the start tile itself
                if i == self.start.position[1] && j == self.start.position[0] {
                    continue;
                }
                // Check if the tile's list of entries contains the start position
                if self.tiles[i as usize][j as usize]
                    .gates
                    .contains(&self.start.position)
                {
                    entries.push([j, i]);
                }
            }
        }

        entries.sort();

        // Update start entries only if entries vec has enough items
        if entries.len() > 1 {
            self.start.gates[0] = entries[0];
            self.start.gates[1] = entries[1];
        }

        // Update start tile direction
        let direction = match (self.start.gates[0], self.start.gates[1]) {
            ([x, y], [x2, y2]) if x == x2 && y < y2 => Direction::SouthNorth,
            ([x, y], [x2, y2]) if x == x2 && y > y2 => Direction::SouthNorth,
            ([x, y], [x2, y2]) if x < x2 && y == y2 => Direction::EastWest,
            ([x, y], [x2, y2]) if (x < x2 && y > y2) && (self.start.position[1] < y) => {
                Direction::SouthEast
            }
            ([x, y], [x2, y2]) if (x < x2 && y < y2) && (self.start.position[0] > x) => {
                Direction::SouthWest
            }
            ([x, y], [x2, y2]) if x < x2 && y > y2 => Direction::NorthWest,
            ([x, y], [x2, y2]) if x < x2 && y < y2 => Direction::NorthEast,
            _ => Direction::default(),
        };
        self.start.direction = direction;
        self.tiles[self.start.position[1] as usize][self.start.position[0] as usize].direction =
            direction;
    }

    fn walk_loop(&mut self) -> Vec<Coordinates> {
        let mut loop_tiles = Vec::new();
        let mut next = self.start.gates[0];
        loop_tiles.push(next);
        let mut prev = self.start.position;

        while next != self.start.position {
            let tile = &mut self.tiles[next[1] as usize][next[0] as usize];
            tile.is_loop = true;
            let cur = next;
            next = tile.to(prev);
            prev = cur;
            loop_tiles.push(next);
        }
        loop_tiles
    }

    fn ray_cast_tile(&self, from: Coordinates) -> i32 {
        let mut count = 0;
        for i in 0..from[0] {
            let tile = &self.tiles[from[1] as usize][i as usize];
            if tile.is_loop
                && (tile.direction == Direction::SouthNorth
                    || tile.direction == Direction::SouthWest
                    || tile.direction == Direction::SouthEast)
            {
                count += 1;
            }
        }
        if count % 2 == 0 {
            0
        } else {
            1
        }
    }

    fn count_enclosed(&self) -> i32 {
        let mut count = 0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if !tile.is_loop && !tile.is_edge {
                    count += self.ray_cast_tile([x as i32, y as i32]);
                }
            }
        }
        count
    }
}

fn parse_input(input: &Vec<String>) -> Maze {
    let mut tiles: Vec<Vec<Tile>> = Vec::new();
    let mut maze = Maze::default();
    for (y, line) in input.iter().enumerate() {
        let mut row: Vec<Tile> = Vec::new();
        for (x, c) in line.chars().enumerate() {
            let mut tile = match c {
                '-' => Tile::new(
                    [[x as i32 - 1, y as i32], [x as i32 + 1, y as i32]],
                    [x as i32, y as i32],
                    Direction::EastWest,
                ),
                '.' => Tile::new(
                    Default::default(),
                    [x as i32, y as i32],
                    Direction::default(),
                ),
                '7' => Tile::new(
                    [[x as i32, y as i32 + 1], [x as i32 - 1, y as i32]],
                    [x as i32, y as i32],
                    Direction::SouthWest,
                ),
                'F' => Tile::new(
                    [[x as i32, y as i32 + 1], [x as i32 + 1, y as i32]],
                    [x as i32, y as i32],
                    Direction::SouthEast,
                ),
                'J' => Tile::new(
                    [[x as i32, y as i32 - 1], [x as i32 - 1, y as i32]],
                    [x as i32, y as i32],
                    Direction::NorthWest,
                ),
                'L' => Tile::new(
                    [[x as i32, y as i32 - 1], [x as i32 + 1, y as i32]],
                    [x as i32, y as i32],
                    Direction::NorthEast,
                ),
                'S' => Tile::start_tile([x as i32, y as i32]),
                '|' => Tile::new(
                    [[x as i32, y as i32 - 1], [x as i32, y as i32 + 1]],
                    [x as i32, y as i32],
                    Direction::SouthNorth,
                ),
                _ => panic!("Unknown tile type: {}", c),
            };
            if tile.is_start {
                maze.start = tile;
            }
            if x == 0 || x == line.len() - 1 || y == 0 || y == input.len() - 1 {
                tile.is_edge = true;
            }
            row.push(tile);
        }
        tiles.push(row);
    }
    maze.tiles = tiles;
    maze.find_start_gates();
    maze
}

pub fn part_1(input: &str) -> i32 {
    let lines = input.lines().map(|line| line.to_string()).collect();
    let mut maze = parse_input(&lines);
    let walk = maze.walk_loop();
    walk.len() as i32 / 2
}

fn part_2(input: &str) -> i32 {
    let lines = input.lines().map(|line| line.to_string()).collect();
    let mut maze = parse_input(&lines);
    maze.walk_loop();
    maze.count_enclosed()
}

#[aoc::main(10)]
fn main(input: &str) -> (usize, usize) {
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    (0, 0)
}
