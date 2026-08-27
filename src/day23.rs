use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};
use enum_iterator::{all, Sequence};

use crate::day::{Day, Answer};

#[derive(Debug, PartialEq, Sequence, Copy, Clone)]
enum Direction {
    East,
    North,
    West,
    South,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::South => Direction::North,
        }
    }
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

    // Move one space in the requested direction
    fn one_move(&self, start: &(usize, usize), dir: &Direction, slippery: bool) -> Option<(usize, usize)> {
        // Get the next coord in that direction
        let to_coord = match dir {
            Direction::North => {
                if start.1 > 0 { Some((start.0, start.1-1)) }
                else { None }
            }
            Direction::South => {
                if start.1 < self.height-1 { Some((start.0, start.1+1)) }
                else { None }
            }
            Direction::East => {
                if start.0 < self.width-1 { Some((start.0+1, start.1)) }
                else { None }
            }
            Direction::West => {
                if start.0 > 0 { Some((start.0-1, start.1)) }
                else { None }
            }
        };

        // Check whether the destination coordinate is valid
        if let Some(to_coord) = to_coord {
            let valid_mode = match &self.map[&to_coord] {
                CellType::Path => true,
                CellType::Forest => false,
                CellType::Slope(slope) => {
                    // Can't step against a slope if it's slippery
                    (*slope != dir.opposite()) || !slippery
                }
            };

            if valid_mode {
                // Valid move - return the desination coordinate
                Some(to_coord)
            }
            else {
                // Invalid move due to "terrain"
                None
            }
        }
        else {
            // Invalid move as it went off the map.
            None
        }
    }

    // Get a vector of all the directions one can move from a given position, excluding the
    // direction that is opposite last_dir, as that would back-track.
    fn directions_from(&self, position: &(usize, usize), last_dir: &Direction, slippery: bool) -> Vec<Direction> {
        all::<Direction>()
            .filter(|d| {
                // Check steep slope condition
                let slope_ok = if let CellType::Slope(slope_dir) = &self.map[position] {
                    (*d == *slope_dir) || !slippery
                }
                else {
                    true
                };

                // This direction is OK if slope condition is met, it isn't in the opposite direction as last move and
                // movement in that direction is to a valid place.
                slope_ok &&
                (*d != last_dir.opposite()) &&
                self.one_move(position, d, slippery).is_some()
            })
            .collect()
    }

    fn intersections(&self) -> (HashMap<(usize, usize), usize>, Vec<(usize, usize)>) {
        let mut node_ids = HashMap::new();
        let mut node_locations = Vec::new();

        // register start node
        let n = node_ids.len();
        node_ids.insert(self.start, n);
        node_locations.push(self.start);

        // register end node
        let n = node_ids.len();
        node_ids.insert(self.end, n);
        node_locations.push(self.end);

        // Scan the whole map looking for intersections
        for (location, cell_type) in &self.map {
            // intersections are interior locations that are not forest and have three or more neighbors that are not forest
            if (location.0 > 0) && (location.0 < self.width-1) &&
               (location.1 > 0) && (location.1 < self.height-1) &&
               *cell_type != CellType::Forest {
                let mut neighbors = 0;

                for dir in all::<Direction>() {
                    let neighbor = match dir {
                        Direction::North => (location.0, location.1-1),
                        Direction::South => (location.0, location.1+1),
                        Direction::East => (location.0+1, location.1),
                        Direction::West => (location.0-1, location.1),
                    };
                    if self.map[&neighbor] != CellType::Forest {
                        neighbors += 1;
                    }
                }

                if neighbors >= 3 {
                    // Location <location> is an intersection
                    let n = node_ids.len();
                    node_ids.insert(*location, n);
                    node_locations.push(*location);
                }
            }
        }     

        (node_ids, node_locations)
    }

    // Take as many steps as possible with the first one in the specified direction.
    // Stop when an intersection is reached, returning position (x, y), num_steps
    // Returns None if a dead end is reached.
    fn follow(&self, start: &(usize, usize), dir: &Direction, intersections: &HashMap<(usize, usize), usize>, slippery: bool) -> Option<(usize, usize, usize)> {
        if let Some(first_step) = self.one_move(start, dir, slippery) {
            let mut position = first_step;
            let mut last_dir = *dir;
            let mut steps = 1;

            loop {
                // If we are now at an intersection (or start or end), terminate the search
                if intersections.contains_key(&position) {
                    return Some((position.0, position.1, steps));
                }

                // Get all the directions one can move from here (not backtracking)
                let directions = self.directions_from(&position, &last_dir, slippery);
                if directions.len() < 1 {
                    // Dead end reached.
                    return None;
                }
                else if directions.len() == 1 {
                    // Exactly one direction we can go, follow it
                    last_dir = directions[0];
                    position = self.one_move(&position, &last_dir, slippery).unwrap();
                    steps += 1;
                }
                else {
                    panic!("Multiple ways to go from a non-intersection!");
                }
            }
        }
        else {
            // First step failed, there is no path in this direction
            None
        }
    }
}

struct Graph {
    start: usize,                       // index in nodes of start node
    end: usize,                         // index in nodes of end node
    arcs: Vec<(usize, usize, usize)>,   // paths (from-node, to-node, steps)
}

impl Graph {
    pub fn from_input(input: &Input, slippery: bool) -> Graph {
        // create vector of arcs (from_node_id, to_node_id, length)
        let mut arcs = Vec::new();

        let (node_ids, node_locations) = input.intersections();

        // Explore from each intersection and record which other intersections are reached
        // and what distance away they are.
        for explore_node in 0..node_locations.len() {
            let from_coord = node_locations[explore_node];

            // Try to step in each of the four cardinal directions, then continue
            // on until we reach the start node, end node or an intersection.
            for dir in all::<Direction>() {
                if let Some((x, y, dist)) = input.follow(&from_coord, &dir, &node_ids, slippery) {                  

                    // Record the path we just took from explore_node to node_id in dist steps
                    let node_id = node_ids[&(x, y)];
                    arcs.push((explore_node, node_id, dist));
                }
            }
        }

        Graph { start: 0, end: 1, arcs }
    }

    fn longest_to(&self, node_id: usize, tail: &Vec<usize>) -> Option<usize> {

        if node_id == self.start {
            // It takes zero steps to get to the start
            return Some(0)
        }
      
        // Iterate over all the ways to get to node_id from nodes not in tail
        let origins: Vec<&(usize, usize, usize)> = self.arcs.iter()
            .filter(|arc| {
                // arcs ending at node_id
                (arc.1 == node_id) && !tail.contains(&arc.0)
            }).collect();

        if origins.is_empty() {
            return None;
        }

        // recursively evaluate longest path via each origin.
        let longest = origins.iter()
            .map(|arc| {
                // let mut new_tail = tail.clone();
                let mut local_tail = tail.clone();
                local_tail.push(arc.1);
                local_tail.sort();
                if let Some(longest) = self.longest_to(arc.0, &local_tail) {
                    Some(longest + arc.2)
                }
                else {
                    None
                }
            })
            .filter(|distance| { distance.is_some() })
            .map(|distance| { distance.unwrap() })
            .max();

        longest
    }

    fn longest(&self) -> usize {
        let mut tail = Vec::new();
        self.longest_to(self.end, &mut tail).unwrap()
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
        let input = Input::read(self._input_filename);
        let graph = Graph::from_input(&input, true);
        let longest = graph.longest();

        Answer::Numeric(longest)
    }

    fn part2(&self) -> Answer {
        let input = Input::read(self._input_filename);
        let graph = Graph::from_input(&input, false);
        let longest = graph.longest();

        Answer::Numeric(longest)
    }
}

#[cfg(test)]
mod test {
    use crate::day23::{Day23, Input, Direction, Graph};
    use crate::day::{Day, Answer};

    #[test]
    fn test_input() {
        let input = Input::read("examples/day23_example1.txt");

        assert_eq!(input.map.len(), 23*23);
        assert_eq!(input.width, 23);
        assert_eq!(input.height, 23);
        assert_eq!(input.start, (1, 0));
        assert_eq!(input.end, (21, 22));
    }

    #[test]
    fn test_intersections() {
        let input = Input::read("examples/day23_example1.txt");

        let (node_ids, node_positions) = input.intersections();
        assert_eq!(node_ids.len(), 9);
        assert_eq!(node_positions.len(), 9);

        for n in 0..node_positions.len() {
            let position = node_positions[n];
            assert_eq!(node_ids[&position], n);
        }
        for position in node_ids.keys() {
            let node_id = node_ids[position];
            assert_eq!(node_positions[node_id], *position);
        }
    }

    #[test]
    fn test_follow() {
        let input = Input::read("examples/day23_example1.txt");
        let (node_ids, _node_positions) = input.intersections();

        assert_eq!(input.follow(&input.start, &Direction::South, &node_ids, false), Some((3, 5, 15)));
    }

    #[test]
    fn test_graph() {
        let input = Input::read("examples/day23_example1.txt");
        let graph = Graph::from_input(&input, true);

        assert_eq!(graph.start, 0);
        assert_eq!(graph.end, 1);
        assert_eq!(graph.arcs.len(), 12);
    }

    #[test]
    fn test_longest() {
        let input = Input::read("examples/day23_example1.txt");
        let graph = Graph::from_input(&input, true);

        assert_eq!(graph.longest(), 94);
    }

    #[test]
    fn test_longest_part2() {
        let input = Input::read("examples/day23_example1.txt");
        let graph = Graph::from_input(&input, false);

        assert_eq!(graph.longest(), 154);
    }

    #[test]
    fn test_part1() {
        let d = Day23::new("examples/day23_example1.txt");

        assert_eq!(d.part1(), Answer::Numeric(94));
    }

    #[test]
    fn test_part2() {
        let d = Day23::new("examples/day23_example1.txt");

        assert_eq!(d.part2(), Answer::Numeric(154));
    }
}
