use std::{fs::File, io::{BufReader, BufRead}};

use crate::day::{Day, Answer};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref INSERT_RE: Regex = Regex::new("[a-z]+=([0-9]+)").unwrap();
}

struct Instruction {
    text: String,
}

impl Instruction {
    pub fn new(s: &str) -> Instruction {
        Instruction { text: s.to_string() }
    }

    fn hash_str(s: &str) -> u8 {
        s.as_bytes().iter()
            .fold(0_u8, |a, b| a.wrapping_add(*b).wrapping_mul(17))
    }

    pub fn hash(&self) -> u8 {
        Instruction::hash_str(&self.text)
    }

    pub fn box_no(&self) -> usize {
        let mut n = 0;
        for c in self.text.chars() {
            if c >= 'a' && c <= 'z' {
                n += 1;
            }
            else {
                break;
            }
        }

        let label = &self.text[0..n];

        Instruction::hash_str(label) as usize
    }

    pub fn label(&self) -> &str {
        let mut n = 0;
        for c in self.text.chars() {
            if c >= 'a' && c <= 'z' {
                n += 1;
            }
            else {
                break;
            }
        }

        let label = &self.text[0..n];

        label
    }

    // If return is Some(N), insert at position N.
    // if return is None, do a remove instead of insert.
    pub fn insert_pos(&self) -> Option<usize> {
        let captures = INSERT_RE.captures(&self.text);
        match captures {
            Some(caps) => {
                // Insert
                let position = caps[1].parse::<usize>().unwrap();
                Some(position)
            },
            None => {
                // Treat as removal
                None
            },
        }
    }
}

struct Input {
    instructions: Vec<Instruction>,
}

impl Input {
    fn read(filename: &str) -> Input {
        let f = File::open(filename).unwrap();
        let mut reader = BufReader::new(f);

        let mut s = String::new();
        reader.read_line(&mut s).unwrap();

        let mut instructions: Vec<Instruction> = Vec::new();
        for i_str in s.split(",") {
            instructions.push(Instruction::new(i_str.trim()));
        }

        Input { instructions }
    }

    /*
    fn hash_label(instr: &str) -> u8 {
        instr.as_bytes().iter()
            .filter(|byte| byte >= 'a' && byte <= 'z')
            .fold(0, |a, b| (a.wrapping_add(*b).wrapping_mul(17)))
    }

    fn hash_inst(instr: &str) -> u8 {
        instr.as_bytes().iter()
            .fold(0, |a, b| (a.wrapping_add(*b).wrapping_mul(17)))
    }
    */

    fn hash_sum(&self) -> usize {
        self.instructions.iter().map(|i| i.hash() as usize).sum()
    }

    fn focusing_power(&self) -> usize {
        // Each box is a vector of (label, focal length)
        const EMPTY_BOX: Vec<(&str, usize)> = vec![];
        let mut boxes: [Vec<(&str, usize)>; 256] = [EMPTY_BOX; 256];

        // Go though instructions, moving lenses around
        for i in &self.instructions {
            let box_no = i.box_no();
            let label = i.label();
            match i.insert_pos() {
                Some(focal_length) => {
                    // Add lens with this label, replacing existing or at end
                    let mut index = 0;
                    for (label, _fl) in &boxes[box_no] {
                        if *label == i.label() { 
                            break; 
                        }
                        index += 1;
                    }
                    if index < boxes[box_no].len() {
                        // replace entry at index
                        boxes[box_no][index] = (label, focal_length);
                    }
                    else {
                        // append entry to the end
                        boxes[box_no].push( (label, focal_length) );
                    }
                }
                None => {
                    // Remove lens with this label (if any)
                    let mut index = 0;
                    for (label, _fl) in &boxes[box_no] {
                        if *label == i.label() { 
                            break; 
                        }
                        index += 1;
                    }
                    if index < boxes[box_no].len() {
                        // remove the element at index
                        boxes[box_no].remove(index);
                    }
                }
            }
        }

        // Add up components of processing power
        let mut power = 0;
        for box_no in 0..boxes.len() {
            for slot in 0..boxes[box_no].len() {
                
                power += (1+box_no) * (1+slot) * boxes[box_no][slot].1;
            }
        }

        power
    }
}

pub struct Day15<'a> {
    input_filename: &'a str,
}

impl<'a> Day15<'a> {
    pub const fn new(filename: &'a str) -> Self {
        Self { input_filename: filename }
    }
}

impl<'a> Day for Day15<'a> {
    fn part1(&self) -> Answer {
        let input = Input::read(self.input_filename);

        Answer::Numeric(input.hash_sum())
    }

    fn part2(&self) -> Answer {
        let input = Input::read(self.input_filename);

        Answer::Numeric(input.focusing_power())
    }
}

#[cfg(test)]
mod tests {
    use crate::{day15::{Input, Day15, Instruction}, day::{Day, Answer}};

    #[test]
    fn test_hash() {
        let i = Instruction::new("HASH");

        assert_eq!(i.hash(), 52_u8);
    }

    #[test]
    fn test_box_no() {
        let cases = vec![
            ("rn=1", 0),
            ("cm-", 0),
            ("qp=3", 1),
            ("cm=2", 0),
            ("qp-", 1),
            ("pc=4", 3),
            ("ot=9", 3),
            ("ab=5", 3),
            ("pc-", 3),
            ("pc=6", 3),
            ("ot=7", 3),
        ];

        for (text, box_no) in cases {
            assert_eq!(Instruction::new(text).box_no(), box_no);
        }
    }

    #[test]
    fn test_insert_pos() {
        let cases = vec![
            ("rn=1", Some(1)),
            ("cm-", None),
            ("qp=3", Some(3)),
            ("cm=2", Some(2)),
            ("qp-", None),
            ("pc=4", Some(4)),
            ("ot=9", Some(9)),
            ("ab=5", Some(5)),
            ("pc-", None),
            ("pc=6", Some(6)),
            ("ot=7", Some(7)),
        ];

        for (text, result) in cases {
            assert_eq!(Instruction::new(text).insert_pos(), result);
        }
    }

    #[test]
    fn test_input() {
        let input = Input::read("examples/day15_example1.txt");

        assert_eq!(input.instructions.len(), 11);
    }

    #[test]
    fn test_hash_sum() {
        let input = Input::read("examples/day15_example1.txt");

        assert_eq!(input.hash_sum(), 1320);
    }

    #[test]
    fn test_power() {
        let input = Input::read("examples/day15_example1.txt");

        assert_eq!(input.focusing_power(), 145);
    }


    #[test]
    fn test_part1() {
        let d = Day15::new("examples/day15_example1.txt");

        assert_eq!(d.part1(), Answer::Numeric(1320));
    }
}
