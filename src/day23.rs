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
    fn one_move(&self, start: &(usize, usize), dir: &Direction) -> Option<(usize, usize)> {
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
                    // Can't step against a slope.
                    *slope != dir.opposite()
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
    fn directions_from(&self, position: &(usize, usize), last_dir: &Direction) -> Vec<Direction> {
        all::<Direction>()
            .filter(|d| {
                // Check steep slope condition
                let slope_ok = if let CellType::Slope(slope_dir) = &self.map[position] {
                    *d == *slope_dir
                }
                else {
                    true
                };

                // This direction is OK if slope condition is met, it isn't in the opposite direction as last move and
                // movement in that direction is to a valid place.
                slope_ok &&
                (*d != last_dir.opposite()) &&
                self.one_move(position, d).is_some()
            })
            .collect()
    }

    // Take as many steps as possible with the first one in the specified direction.
    // Stop when an intersection is reached, returning x, y, num_steps
    // Returns None if a dead end is reached.
    fn follow(&self, start: &(usize, usize), dir: &Direction) -> Option<(usize, usize, usize)> {
        if let Some(first_step) = self.one_move(start, dir) {
            let mut position = first_step;
            let mut last_dir = *dir;
            let mut steps = 1;

            loop {
                // Get all the directions one can move from here (not backtracking)
                let directions = self.directions_from(&position, &last_dir);
                if directions.len() < 1 {
                    // Dead end reached.
                    return None;
                }
                else if directions.len() > 1 {
                    // We reached an intersection
                    return Some((position.0, position.1, steps));
                }
                else {
                    // Exactly one direction we can go, follow it
                    last_dir = directions[0];
                    position = self.one_move(&position, &last_dir).unwrap();
                    steps += 1;
                }
            }
        }
        else {
            // First step failed, there is no path in this direction
            None
        }
    }

    /*
    // Given a map, location and the set of already visited cells, which directions can we go?
    fn dir_options(&self, visited: &HashSet<(usize, usize)>, location: (usize, usize)) -> HashSet<Direction> {
        
    }
    */

    /*
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
    */

    /*
    fn longest_path(&self) -> Vec<Direction> {
        let path = vec![Direction::South];

        let longest = // self.explore(&path).unwrap();

        longest
    }
    */
}

struct Graph {
    start: usize,                       // index in nodes of start node
    end: usize,                         // index in nodes of end node
    nodes: Vec<(usize, usize)>,         // identified nodes by (x, y) coord
    paths: Vec<(usize, usize, usize)>,  // paths (from-node, to-node, steps)
}

impl Graph {
    pub fn from_input(input: &Input) -> Graph {
        // TODO-DW : Fix problem where converging nodes aren't recognized.
        // TODO-DW : Fix problem where end node isn't recognized.

        // create empty nodes and paths 
        let mut node_ids: HashMap<(usize, usize), usize> = HashMap::new();  // (x, y) -> node id
        let mut node_locations: Vec<(usize, usize)> = Vec::new();           // node id -> (x, y)
        let mut paths = Vec::new();

        // register start node
        let n = node_ids.len();
        node_ids.insert(input.start, n);
        node_locations.push(input.start);

        // register end node
        let n = node_ids.len();
        node_ids.insert(input.end, n);
        node_locations.push(input.end);

        // nodes to explore from
        let mut to_explore = vec![0];

        while let Some(explore_node) = to_explore.pop() {
            let from_coord = node_locations[explore_node];

            // Try to step in each of the four cardinal directions, then continue
            // on until we reach the start node, end node or a node with a branch.
            for dir in all::<Direction>() {
                println!("Following {dir:?} from {}:({}, {})", explore_node, from_coord.0, from_coord.1);
                if let Some((x, y, dist)) = input.follow(&from_coord, &dir) {
                    // If we ended somewhere new, register a new node and push it to to_explore list
                    let node_id = if !node_ids.contains_key(&(x, y)) {
                        // Found a new node to explore
                        let node_id = node_ids.len();
                        node_ids.insert((x, y), n);
                        node_locations.push((x, y));                        
                        to_explore.push(node_id);

                        println!("  Got to new node: {}:({}, {})", node_id, x, y);
                        node_id
                    }
                    else {
                        println!("  Got to existing node: {}:({}, {})", node_ids[&(x, y)], x, y);
                        node_ids[&(x, y)]
                    };

                    // Record the path we just took from explore_node to node_id in dist steps
                    paths.push((explore_node, node_id, dist));
                }
                else {
                    println!("  Got nowhere")
                }
            }
        }

        Graph { start: 0, end: 1, nodes: node_locations, paths }
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
    use crate::day23::{Input, Direction, Graph};

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
    fn test_follow() {
        let input = Input::read("examples/day23_example1.txt");

        assert_eq!(input.follow(&input.start, &Direction::South), Some((3, 5, 15)));
    }

    #[test]
    fn test_graph() {
        let input = Input::read("examples/day23_example1.txt");
        let graph = Graph::from_input(&input);


        for n in 0..6 {
            println!("Node {n}: ({}, {})", graph.nodes[n].0, graph.nodes[n].1);
        }

        for n in 0..5 {
            println!("Path {n}: from {} to {}, len {}", graph.paths[n].0, graph.paths[n].1, graph.paths[n].2);
        }


        assert_eq!(graph.start, 0);
        assert_eq!(graph.end, 1);
        assert_eq!(graph.nodes.len(), 9);
        assert_eq!(graph.paths.len(), 5);

    }
}
