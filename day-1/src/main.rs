use std::fs::File;
use std::io::Read;
use std::io::Result;

struct Locker {
    position: i32,
    clicks: i32,
}

impl Locker {
    fn new() -> Locker {
        Locker {
            position: 50,
            clicks: 0,
        }
    }
    fn turn_right(&mut self, number: i32, count_all_clicks: bool) {
        let mut previous_position = self.position;
        let turns = number / 100;
        let corrected_number = number % 100;
        self.position += corrected_number;
        if self.position == 0 {
            self.clicks += 1
        }
        while self.position > 99 {
            self.position -= 100;
            if count_all_clicks && self.position != 0 && previous_position != 0 {
                self.clicks += 1;
            }
            if self.position == 0 {
                self.clicks += 1;
            }
            previous_position = self.position;
        }
        if count_all_clicks && number > 100 {
            self.clicks += turns;
        }
    }

    fn turn_left(&mut self, number: i32, count_all_clicks: bool) {
        let previous_position = self.position;
        let turns = number / 100;
        let corrected_number = number % 100;
        self.position -= corrected_number;
        if self.position == 0 {
            self.clicks += 1
        }
        while self.position < 0 {
            self.position += 100;
            if count_all_clicks && self.position != 0 && previous_position != 0 {
                self.clicks += 1;
            }
            if self.position == 0 {
                self.clicks += 1;
            }
        }
        if count_all_clicks && number > 100 {
            self.clicks += turns;
        }
    }

    fn password(self) -> i32 {
        self.clicks
    }
}

fn read_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn split_str(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

fn part_1() {
    let file_content = read_file("src/1.txt").unwrap();
    let mut l = Locker::new();
    let file_content: Vec<&str> = file_content.split("\n").collect();

    for command in file_content {
        let (first_char, remainder) = split_str(command);
        if !first_char.is_empty() {
            let number: i32 = remainder.parse().unwrap();
            match first_char {
                "L" => l.turn_left(number, false),
                "R" => l.turn_right(number, false),
                &_ => eprintln!("Error!"),
            }
        }
    }

    println!("First challenge part 1: {:?}", l.password())
}

fn part_2() {
    let file_content = read_file("src/1.txt").unwrap();
    let mut l = Locker::new();
    let file_content: Vec<&str> = file_content.split("\n").collect();

    for command in file_content {
        // println!("{:?}", command);
        // println!("position before command: {:?}", l.position);
        // println!("clicks before command: {:?}", l.clicks);
        let (first_char, remainder) = split_str(command);
        if !first_char.is_empty() {
            let number: i32 = remainder.parse().unwrap();
            match first_char {
                "L" => l.turn_left(number, true),
                "R" => l.turn_right(number, true),
                &_ => eprintln!("Error!"),
            }
        }
        // println!("position after command: {:?}", l.position);
        // println!("clicks after command: {:?}", l.clicks);
    }

    println!("First challenge part 2: {:?}", l.password())
}

fn main() {
    part_1();
    part_2();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_right_no_count_all() {
        let mut l = Locker::new();
        l.turn_right(100, false);

        assert_eq!(l.position, 50);
        assert_eq!(l.clicks, 0);
        assert_eq!(l.password(), 0);
    }

    #[test]
    fn test_turn_right_no_count_all_zero() {
        let mut l = Locker::new();
        l.turn_right(50, false);

        assert_eq!(l.position, 0);
        assert_eq!(l.clicks, 1);
        assert_eq!(l.password(), 1);
    }

    #[test]
    fn test_turn_left_no_count_all() {
        let mut l = Locker::new();
        l.turn_left(100, false);

        assert_eq!(l.position, 50);
        assert_eq!(l.clicks, 0);
        assert_eq!(l.password(), 0);
    }

    #[test]
    fn test_turn_right_count_all() {
        let mut l = Locker::new();
        l.turn_right(1000, true);

        assert_eq!(l.position, 50);
        assert_eq!(l.clicks, 10);
        assert_eq!(l.password(), 10);
    }

    #[test]
    fn test_turn_right_count_all_start_at_zero() {
        let mut l = Locker::new();
        // create 10 clicks
        l.turn_right(1000, true);

        assert_eq!(l.position, 50);
        assert_eq!(l.clicks, 10);
        assert_eq!(l.password(), 10);
    }
    #[test]
    fn test_turn_left_count_all_start_at_zero() {
        let mut l = Locker::new();
        // create 10 clicks
        l.turn_left(1000, true);

        assert_eq!(l.position, 50);
        assert_eq!(l.clicks, 10);
        assert_eq!(l.password(), 10);
    }

    #[test]
    fn test_zero_convertion_lr() {
        let mut l = Locker::new();
        l.turn_left(50, true);

        l.turn_left(1, true);
        l.turn_right(1, true);

        assert_eq!(l.position, 0);
        assert_eq!(l.clicks, 2);
    }

    #[test]
    fn test_zero_convertion_rl() {
        let mut l = Locker::new();
        l.turn_left(50, true);

        l.turn_right(1, true);
        l.turn_left(1, true);

        assert_eq!(l.position, 0);
        assert_eq!(l.clicks, 2);
    }
    #[test]
    fn test_sequence() {
        let mut l = Locker::new();
        // 10
        l.turn_right(1000, true);
        // 10
        l.turn_left(1000, true);

        // 1
        l.turn_left(50, true);

        // 1
        l.turn_right(1, true);
        l.turn_left(1, true);

        // 1
        l.turn_left(1, true);
        l.turn_right(1, true);

        // 1
        l.turn_right(100, true);
        l.turn_right(1, true);

        assert_eq!(l.position, 1);
        assert_eq!(l.clicks, 24);
    }

    #[test]
    fn test_l50_r101() {
        let mut l = Locker::new();
        // 1
        l.turn_left(50, true);

        //1 ?
        l.turn_right(101, true);

        assert_eq!(l.position, 1);
        assert_eq!(l.clicks, 2);
    }

    #[test]
    fn test_l50_r100() {
        let mut l = Locker::new();
        // 1
        l.turn_left(50, true);

        //1 ?
        l.turn_right(100, true);

        assert_eq!(l.position, 0);
        assert_eq!(l.clicks, 2);
    }
}
