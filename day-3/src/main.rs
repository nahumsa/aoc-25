use std::fs::File;
use std::io::Read;
use std::io::Result;

fn read_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn find_max_joltage(joltage: &str) -> i64 {
    let mut a = 0;
    let mut b = 0;

    for (i, val) in joltage.chars().enumerate() {
        let val_int = val.to_digit(10).unwrap();
        if val_int > a && i < joltage.chars().count() - 1 {
            a = val_int;
            b = 0;
        } else if val_int > b {
            b = val_int
        }
    }
    format!("{}{}", a, b).parse().unwrap()
}

fn part_1(file_content: Vec<&str>) -> i64 {
    let mut total = 0;
    for val in file_content {
        total += find_max_joltage(val)
    }
    total
}

fn main() {
    let file_content = read_file("src/3.txt").unwrap();
    let file_content: Vec<&str> = file_content.split("\n").collect();

    println!("Third challenge part 1: {:?}", part_1(file_content.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_max_joltage_1() {
        let joltage = "987654321111111";

        let invalid = find_max_joltage(joltage);

        assert_eq!(invalid, 98);
    }
    #[test]
    fn test_find_max_joltage_2() {
        let joltage = "811111111111119";

        let invalid = find_max_joltage(joltage);

        assert_eq!(invalid, 89);
    }

    #[test]
    fn test_find_max_joltage_3() {
        let joltage = "234234234234278";

        let invalid = find_max_joltage(joltage);

        assert_eq!(invalid, 78);
    }

    #[test]
    fn test_find_max_joltage_4() {
        let joltage = "818181911112111";

        let invalid = find_max_joltage(joltage);

        assert_eq!(invalid, 92);
    }
}
