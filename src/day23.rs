use std::{collections::{HashMap, HashSet}, fs::File, io::{BufRead, BufReader}};

use crate::day::{Day, Answer};

#[derive(Debug, PartialEq)]
enum Direction {
    East,
    North,
    West,
    South,
}

#[derive(Debug, PartialEq)]
enum CellType {
    Path,
    Forest,
    Slope(Direction),
}

struct Input {
    map: HashMap<(usize, usize), CellType>,
    width: usize,
    height: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl Input {
    fn read(filename: &str) -> Input {
        let mut map = HashMap::new();
        let mut width= 0;
        let mut height = 0;

        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);

        let mut y = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let mut x = 0;

            for c in line.chars() {
                match c {
                    '.' => {
                        // Path
                        map.insert((x, y), CellType::Path);
                    }
                    '#' => {
                        // Forest
                        map.insert((x, y), CellType::Forest);
                    }
                    '>' => {
                        // East
                        map.insert((x, y), CellType::Slope(Direction::East));
                    }
                    '^' => {
                        // North
                        map.insert((x, y), CellType::Slope(Direction::North));
                    }
                    '<' => {
                        // West
                        map.insert((x, y), CellType::Slope(Direction::West));
                    }
                    'v' => {
                        // South
                        map.insert((x, y), CellType::Slope(Direction::South));
                    }
                    _ => {
                        panic!("Invalid character, {} in input.", c);
                    }
                }

                x += 1;
                if x > width {
                    width = x;
                }
            }

            y += 1;
            if y > height {
                height = y;
            }
        }

        // Find start in first row, y=0
        let mut start = (0, 0);
        for x in 0..width {
            if map[&(x, 0)] == CellType::Path {
                start = (x, 0);
                break;
            }
        }

        // Find end in last row, y=0
        let mut end = (0, 0);
        for x in 0..width {
            if map[&(x, height-1)] == CellType::Path {
                end = (x, height-1);
                break;
            }
        }

        Input { map, width, height, start, end }
    }

    // Given a map, location and the set of already visited cells, which directions can we go?
    fn dir_options(&self, visited: &HashSet<(usize, usize)>, location: (usize, usize)) -> HashSet<Direction> {
        
    }

    // Starting with a partial path, find longest path from here.
    fn explore(&self, path: &Vec<Direction>) -> Option<Vec<Direction>> {
        // Trace the path, constructing a coverage map and current location.
        let mut location = self.start;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        visited.insert(location);

        for direction in path {
            location = match direction {
                Direction::East => (location.0+1, location.1),
                Direction::North => (location.0, location.1-1),
                Direction::West => (location.0-1, location.1),
                Direction::South => (location.0, location.1+1),
            };
            visited.insert(location);
        }

        // TODO: Evaluate all the directions the path could go next.
        let options: Vec<Direction> = Vec::new();
        for 
        // TODO: While there is only one way to go, add it to the path, re-evaluate options.
        // TODO: If no choices on where to go
        // TODO:     If at end, return this path
        // TODO:     Else, return None
        // TODO: Else: (multiple choices)
        // TODO:     call explore recursively with each option
        // TODO:     return longest of options available

        None

    }

    fn longest_path(&self) -> Vec<Direction> {
        let path = vec![Direction::South];

        let longest = self.explore(&path).unwrap();

        longest
    }
}

pub struct Day23<'a> {
    _input_filename: &'a str,
}

impl<'a> Day23<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { _input_filename: filename }
    }
}

impl<'a> Day for Day23<'a> {
    fn part1(&self) -> Answer {
        Answer::None
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}

#[cfg(test)]
mod test {
    use crate::day23::Input;

    #[test]
    fn test_input() {
        let input = Input::read("examples/day23_example1.txt");

        assert_eq!(input.map.len(), 23*23);
        assert_eq!(input.width, 23);
        assert_eq!(input.height, 23);
        assert_eq!(input.start, (1, 0));
        assert_eq!(input.end, (21, 22));
    }


}
