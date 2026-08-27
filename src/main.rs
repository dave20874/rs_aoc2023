mod day;
mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;


use day::{Day, Answer};
use day0::Day0;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day20::Day20;
use day21::Day21;
use day22::Day22;
use day23::Day23;
use day24::Day24;
use day25::Day25;



fn report_day(day: &dyn Day, day_no: usize) {
    
    let ans1 = day.part1();
    let msg1 = match ans1 {
        Answer::None => String::from("No Answer"),
        Answer::Numeric(n) => format!("{n}"),
        Answer::String(s) => format!("{s}"),
    };

    let ans2 = day.part2();
    let msg2 = match ans2 {
        Answer::None => String::from("No Answer"),
        Answer::Numeric(n) => format!("{n}"),
        Answer::String(s) => format!("{s}"),
    };
    println!("Day {day_no:2}: {msg1:>16} {msg2:>16}");
}

static DAYS: [&dyn Day; 26] = [
    &Day0::new("data_aoc2023/day0.txt"),  // Placeholder
    &Day1::new("data_aoc2023/day1.txt"),  // Dec 1
    &Day2::new("data_aoc2023/day2.txt"),
    &Day3::new("data_aoc2023/day3.txt"),
    &Day4::new("data_aoc2023/day4.txt"),
    &Day5::new("data_aoc2023/day5.txt"),  // Dec 5
    &Day6::new("data_aoc2023/day6.txt"),
    &Day7::new("data_aoc2023/day7.txt"),
    &Day8::new("data_aoc2023/day8.txt"),
    &Day9::new("data_aoc2023/day9.txt"),
    &Day10::new("data_aoc2023/day10.txt"),  // Dec 10
    &Day11::new("data_aoc2023/day11.txt"),
    &Day12::new("data_aoc2023/day12.txt"),
    &Day13::new("data_aoc2023/day13.txt"),
    &Day14::new("data_aoc2023/day14.txt"),
    &Day15::new("data_aoc2023/day15.txt"),  // Dec 15
    &Day16::new("data_aoc2023/day16.txt"),
    &Day17::new("data_aoc2023/day17.txt"),
    &Day18::new("data_aoc2023/day18.txt"),
    &Day19::new("data_aoc2023/day19.txt"),
    &Day20::new("data_aoc2023/day20.txt"),  // Dec 20
    &Day21::new("data_aoc2023/day21.txt"),
    &Day22::new("data_aoc2023/day22.txt"),
    &Day23::new("data_aoc2023/day23.txt"),
    &Day24::new("data_aoc2023/day24.txt"),
    &Day25::new("data_aoc2023/day25.txt"),  // Dec 25
];

fn main() {
    println!("Advent of Code 2023!\n");

    let target_day = 0;

    match target_day {
        0 => {
            // report all days
            println!("{:7} {:>16} {:>16}", "", "Part 1", "Part 2");
            for day_no in 1..=25 {
                report_day(DAYS[day_no], day_no);
            }
        }
        1..=25 => {
            // report a specific day
            report_day(DAYS[target_day], target_day);
        }
        _ => {
            // invalid day 
            println!("Day {target_day} is invalid.\n");
        }
    }
    println!();

}

#[cfg(test)]
mod test {
    use crate::day::{Day, Answer};
    use crate::DAYS;

    #[test]
    fn test_day0_part1() {
        let d: &dyn Day = DAYS[0];
        assert_eq!(d.part1(), Answer::None);
    }

    #[test]
    fn test_day0_part2() {
        let d: &dyn Day = DAYS[0];
        assert_eq!(d.part2(), Answer::None);
    }

    #[test]
    fn test_day1_part1() {
        let d: &dyn Day = DAYS[1];
        assert_eq!(d.part1(), Answer::Numeric(55029));
    }

    #[test]
    fn test_day1_part2() {
        let d: &dyn Day = DAYS[1];
        assert_eq!(d.part2(), Answer::Numeric(55686));  // Not 55680
    }

    #[test]
    fn test_day2_part1() {
        let d: &dyn Day = DAYS[2];
        assert_eq!(d.part1(), Answer::Numeric(1853));
    }

    #[test]
    fn test_day2_part2() {
        let d: &dyn Day = DAYS[2];
        assert_eq!(d.part2(), Answer::Numeric(72706));
    }

    #[test]
    fn test_day3_part1() {
        let d: &dyn Day = DAYS[3];
        assert_ne!(d.part1(), Answer::Numeric(508789));
        assert_eq!(d.part1(), Answer::Numeric(525911));
    }

    #[test]
    fn test_day3_part2() {
        let d: &dyn Day = DAYS[3];
        assert_eq!(d.part2(), Answer::Numeric(75805607));
    }

    #[test]
    fn test_day4_part1() {
        let d: &dyn Day = DAYS[4];
        assert_eq!(d.part1(), Answer::Numeric(24160));
    }

    #[test]
    fn test_day4_part2() {
        let d: &dyn Day = DAYS[4];
        assert_eq!(d.part2(), Answer::Numeric(5659035));
    }
    
    #[test]
    fn test_day5_part1() {
        let d: &dyn Day = DAYS[5];
        assert_eq!(d.part1(), Answer::Numeric(51752125));
    }
    
    #[test]
    fn test_day5_part2() {
        let d: &dyn Day = DAYS[5];
        assert_eq!(d.part2(), Answer::Numeric(12634632));
    }
        
    #[test]
    fn test_day6_part1() {
        let d: &dyn Day = DAYS[6];
        assert_eq!(d.part1(), Answer::Numeric(840336));
    }
    
    #[test]
    fn test_day6_part2() {
        let d: &dyn Day = DAYS[6];
        assert_eq!(d.part2(), Answer::Numeric(41382569));
    }
        
    #[test]
    fn test_day7_part1() {
        let d: &dyn Day = DAYS[7];
        assert_eq!(d.part1(), Answer::Numeric(255048101));
    }
    
    #[test]
    fn test_day7_part2() {
        let d: &dyn Day = DAYS[7];
        assert_eq!(d.part2(), Answer::Numeric(253718286));
    }
        
    #[test]
    fn test_day8_part1() {
        let d: &dyn Day = DAYS[8];
        assert_eq!(d.part1(), Answer::Numeric(19783));
    }
    
    #[test]
    fn test_day8_part2() {
        let d: &dyn Day = DAYS[8];
        assert_eq!(d.part2(), Answer::Numeric(9177460370549));
    }
        
    #[test]
    fn test_day9_part1() {
        let d: &dyn Day = DAYS[9];
        assert_eq!(d.part1(), Answer::Numeric(1955513104));
    }
    
    #[test]
    fn test_day9_part2() {
        let d: &dyn Day = DAYS[9];
        assert_eq!(d.part2(), Answer::Numeric(1131));
    }
        
    #[test]
    fn test_day10_part1() {
        let d: &dyn Day = DAYS[10];
        assert_eq!(d.part1(), Answer::Numeric(6860));
    }
    
    #[test]
    fn test_day10_part2() {
        let d: &dyn Day = DAYS[10];
        assert_eq!(d.part2(), Answer::Numeric(343));
    }
        
    #[test]
    fn test_day11_part1() {
        let d: &dyn Day = DAYS[11];
        assert_eq!(d.part1(), Answer::Numeric(9799681));
    }
    
    #[test]
    fn test_day11_part2() {
        let d: &dyn Day = DAYS[11];
        assert_eq!(d.part2(), Answer::Numeric(513171773355));
    }
        
    #[test]
    fn test_day12_part1() {
        let d: &dyn Day = DAYS[12];
        assert_eq!(d.part1(), Answer::Numeric(7221));
    }
    
    #[test]
    fn test_day12_part2() {
        let d: &dyn Day = DAYS[12];
        assert_eq!(d.part2(), Answer::Numeric(7139671893722));
    }
        
    #[test]
    fn test_day13_part1() {
        let d: &dyn Day = DAYS[13];
        assert_eq!(d.part1(), Answer::Numeric(33047));
    }
    
    #[test]
    fn test_day13_part2() {
        let d: &dyn Day = DAYS[13];
        assert_eq!(d.part2(), Answer::Numeric(28806));
    }
        
    #[test]
    fn test_day14_part1() {
        let d: &dyn Day = DAYS[14];
        assert_eq!(d.part1(), Answer::Numeric(113456));
    }
    
    #[test]
    fn test_day14_part2() {
        let d: &dyn Day = DAYS[14];
        assert_eq!(d.part2(), Answer::Numeric(118747));
    }
        
    #[test]
    fn test_day15_part1() {
        let d: &dyn Day = DAYS[15];
        assert_eq!(d.part1(), Answer::Numeric(515495));
    }
    
    #[test]
    fn test_day15_part2() {
        let d: &dyn Day = DAYS[15];
        assert_eq!(d.part2(), Answer::Numeric(229349));
    }
        
    #[test]
    fn test_day16_part1() {
        let d: &dyn Day = DAYS[16];
        assert_eq!(d.part1(), Answer::Numeric(7951));
    }
    
    #[test]
    fn test_day16_part2() {
        let d: &dyn Day = DAYS[16];
        assert_eq!(d.part2(), Answer::Numeric(8148));
    }
        
    #[test]
    fn test_day17_part1() {
        let d: &dyn Day = DAYS[17];
        assert_eq!(d.part1(), Answer::Numeric(1263));
    }
    
    #[test]
    fn test_day17_part2() {
        let d: &dyn Day = DAYS[17];
        assert_eq!(d.part2(), Answer::Numeric(1411)); // 1408 too low achieved by reversing dir.
    }
        
    #[test]
    fn test_day18_part1() {
        let d: &dyn Day = DAYS[18];
        assert_eq!(d.part1(), Answer::Numeric(47527));
    }
    
    #[test]
    fn test_day18_part2() {
        let d: &dyn Day = DAYS[18];
        assert_eq!(d.part2(), Answer::Numeric(52240187443190));
    }
        
    #[test]
    fn test_day19_part1() {
        let d: &dyn Day = DAYS[19];
        assert_eq!(d.part1(), Answer::Numeric(377025));
    }
    
    #[test]
    fn test_day19_part2() {
        let d: &dyn Day = DAYS[19];
        assert_eq!(d.part2(), Answer::Numeric(135506683246673));
    }
        
    #[test]
    fn test_day20_part1() {
        let d: &dyn Day = DAYS[20];
        assert_eq!(d.part1(), Answer::Numeric(730797576));  // > 345110400
    }
    
    #[test]
    fn test_day20_part2() {
        let d: &dyn Day = DAYS[20];
        assert_eq!(d.part2(), Answer::Numeric(226732077152351)); // < 226732077152352
    }
        
    #[test]
    fn test_day21_part1() {
        let d: &dyn Day = DAYS[21];
        assert_eq!(d.part1(), Answer::Numeric(3724));
    }
            
    #[test]
    fn test_day21_part2() {
        let d: &dyn Day = DAYS[21];
        assert_eq!(d.part2(), Answer::Numeric(620348631910321));
    }
    
    #[test]
    fn test_day22_part1() {
        let d: &dyn Day = DAYS[22];

        assert_ne!(d.part1(), Answer::Numeric(483));         // 483 is too high
        assert_ne!(d.part1(), Answer::Numeric(408));         // 408 is too high
        assert_eq!(d.part1(), Answer::Numeric(389));
    }
        
    #[test]
    fn test_day22_part2() {
        let d: &dyn Day = DAYS[22];
        assert_eq!(d.part2(), Answer::Numeric(70609));
    }
        
    #[test]
    fn test_day23_part1() {
        let d: &dyn Day = DAYS[23];
        assert_eq!(d.part1(), Answer::Numeric(2162));
    }
            
    #[test]
    fn test_day23_part2() {
        let d: &dyn Day = DAYS[23];
        assert_eq!(d.part2(), Answer::Numeric(6334));  // 6469 is too high (and 133 seconds is too long.)
    }
    
    #[test]
    fn test_day24_part1() {
        let d: &dyn Day = DAYS[24];
        assert_eq!(d.part1(), Answer::Numeric(27732));
    }    
    #[test]
    fn test_day24_part2() {
        let d: &dyn Day = DAYS[24];
        assert_eq!(d.part2(), Answer::None);
    }
        
    #[test]
    fn test_day25_part1() {
        let d: &dyn Day = DAYS[25];
        assert_eq!(d.part1(), Answer::None);
    }
    
    #[test]
    fn test_day25_part2() {
        let d: &dyn Day = DAYS[25];
        assert_eq!(d.part2(), Answer::None);
    }

}
