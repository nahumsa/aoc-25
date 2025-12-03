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
    fn turn_right(&mut self, number: i32) {
        self.position += number;
        while self.position > 99 {
            self.position -= 100;
        }
        if self.position == 0 {
            self.clicks += 1;
        }
    }

    fn turn_left(&mut self, number: i32) {
        self.position -= number;
        while self.position < 0 {
            self.position += 100;
        }
        if self.position == 0 {
            self.clicks += 1;
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
                "L" => l.turn_left(number),
                "R" => l.turn_right(number),
                &_ => eprintln!("Error!"),
            }
        }
    }

    print!("First challenge part 1: {:?}", l.password())
}

fn main() {
    part_1();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turn_right() {
        let mut l = Locker::new();
        l.turn_right(100);

        assert_eq!(l.position, 50);
        assert_eq!(l.clicks, 1);
        assert_eq!(l.password(), 1);
    }

    #[test]
    fn test_turn_left() {
        let mut l = Locker::new();
        l.turn_left(100);

        assert_eq!(l.position, 50);
        assert_eq!(l.clicks, 1);
        assert_eq!(l.password(), 1);
    }
}
