use std::{fs::File, io::{BufRead, BufReader}};
use itertools::Itertools;

use regex::Regex;

use crate::day::{Day, Answer};

struct Trajectory {
    pos: [i64; 3],
    vel: [i64; 3],
}

struct RegionOfInterest {
    min: f64,
    max: f64,
}

impl Trajectory {
    // Determine whether two 2D trajectories intersect in the region of interest.
    fn crosses_xy(&self, other: &Trajectory, roi: &RegionOfInterest) -> bool {
        // Compute determinant
        let m1 = (self.vel[1] as f64) / (self.vel[0] as f64);
        let b1 = self.pos[1] as f64 - (self.vel[1]*self.pos[0]) as f64/(self.vel[0] as f64);
        let m2 = (other.vel[1] as f64) / (other.vel[0] as f64);
        let b2 = other.pos[1] as f64 - (other.vel[1]*other.pos[0]) as f64/(other.vel[0] as f64);

        if m1 == m2 {
            // Parallel paths (or colinear)
            if b1 == b2 {
                // collinear
                true
            }
            else {
                // parallel
                false
            }
        }
        else if ((m1 > m2) && (b1 > b2)) ||
           ((m2 > m1) && (b2 > b1)) {
            // paths may have crossed in the past but are diverging now.
            false
        }
        else {
            // Non-parallel, compute the x coordinate where they meet
            let x = (b2 - b1) / (m1 - m2);
            let y = m1 * x + b1;
            let t1 = (x - self.pos[0] as f64)/self.vel[0] as f64;
            let t2 = (x - other.pos[0] as f64)/other.vel[0] as f64;

            // Check if (x,y) is in region of interest
            if (x >= roi.min) && (x <= roi.max) &&
               (y >= roi.min) && (y <= roi.max) && 
               (t1 > 0.0) && (t2 > 0.0) {
                // intersection is in region of interest and ahead in time for both trajectories
                true
            }
            else {
                // They intersect but outside the region of interest
                false
            }
        }
    }
}

struct Input {
    trajectories: Vec<Trajectory>,
}

impl Input {
    fn read_file(filename: &str) -> Input {
        let mut trajectories = Vec::new();
        let line_re = Regex::new("([0-9]+), ([0-9]+), ([0-9]+) @ \\s*(-?[0-9]+), \\s*(-?[0-9]+), \\s*(-?[0-9]+)").unwrap();


        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();

            if let Some(caps) = line_re.captures(&line) {
                let p_x = caps[1].parse::<i64>().unwrap();
                let p_y = caps[2].parse::<i64>().unwrap();
                let p_z = caps[3].parse::<i64>().unwrap();
                let v_x = caps[4].parse::<i64>().unwrap();
                let v_y = caps[5].parse::<i64>().unwrap();
                let v_z = caps[6].parse::<i64>().unwrap();

                trajectories.push( Trajectory { pos: [p_x, p_y, p_z], vel: [v_x, v_y, v_z] });
            }
        }

        Input { trajectories }
    }
}

pub struct Day24<'a> {
    _input_filename: &'a str,
}

impl<'a> Day24<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { _input_filename: filename }
    }
}

impl<'a> Day for Day24<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read_file(self._input_filename);
        let roi = RegionOfInterest { min: 200000000000000.0, max: 400000000000000.0 };

        let count = input.trajectories.iter()
            .array_combinations::<2>()
            .filter(|trajectories| {
                trajectories[0].crosses_xy(trajectories[1], &roi)
            })
            .count();

        Answer::Numeric(count)
    }

    fn part2(&self) -> Answer {
        Answer::None
    }
}

#[cfg(test)]
mod test {
    use crate::day24::{Day24, Input, RegionOfInterest};
    use crate::day::{Day, Answer};

    #[test]
    fn test_input() {
        let input = Input::read_file("examples/day24_example1.txt");

        assert_eq!(input.trajectories.len(), 5);
        assert_eq!(input.trajectories[2].pos[0], 20);
        assert_eq!(input.trajectories[3].vel[1], -2);
    }

    #[test]
    fn test_crosses() {
        let input = Input::read_file("examples/day24_example1.txt");
        let roi = RegionOfInterest { min: 7.0, max: 27.0 };

        assert!(input.trajectories[0].crosses_xy(&input.trajectories[1], &roi));
        assert!(input.trajectories[0].crosses_xy(&input.trajectories[2], &roi));
        assert!(!input.trajectories[0].crosses_xy(&input.trajectories[3], &roi));
        assert!(!input.trajectories[0].crosses_xy(&input.trajectories[4], &roi));
        assert!(!input.trajectories[1].crosses_xy(&input.trajectories[2], &roi));
        assert!(!input.trajectories[1].crosses_xy(&input.trajectories[3], &roi));
        assert!(!input.trajectories[1].crosses_xy(&input.trajectories[4], &roi));
        assert!(!input.trajectories[2].crosses_xy(&input.trajectories[3], &roi));
        assert!(!input.trajectories[2].crosses_xy(&input.trajectories[4], &roi));
        assert!(!input.trajectories[3].crosses_xy(&input.trajectories[4], &roi));
    }

    #[test]
    fn test_part1() {
        let d = Day24::new("examples/day24_example1.txt");

        assert_eq!(d.part1(), Answer::Numeric(0));
    }
}

