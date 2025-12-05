use std::fs::File;
use std::io::Read;
use std::io::Result;

fn read_file(filename: &str) -> Result<String> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn check_duplicated(val: String) -> bool {
    let len = val.len() / 2;
    let left = &val[..len];
    let right = &val[len..];
    right.contains(left) && len > 1
}

fn validate_invalid(min_val: String, max_val: String) -> i64 {
    let mut invalid_number = 0;

    let len_min = min_val.len() / 2;
    let left_min = &min_val[..len_min];
    let right_min = &min_val[len_min..];

    let len_max = max_val.len() / 2;
    let left_max = &max_val[..len_max];

    let duplicated_val_min = format!("{}{}", left_min, left_min);
    dbg!("{:?}", left_min);
    let duplicated_number_min: i64 = duplicated_val_min.parse().unwrap_or(0);

    let duplicated_val_max = format!("{}{}", left_max, left_max);
    let duplicated_number_max: i64 = duplicated_val_max.parse().unwrap_or(0);

    let max_number: i64 = max_val.parse().unwrap();
    let min_number: i64 = min_val.parse().unwrap();

    dbg!("{}", duplicated_number_min);
    dbg!("{}", duplicated_number_max);

    if right_min == left_min {
        invalid_number += min_number;
    }

    if min_number + max_number - duplicated_number_min < max_number
        && !check_duplicated(String::from(left_min))
    {
        invalid_number += duplicated_number_min;
    }

    if min_number + max_number - duplicated_number_max < max_number
        && !check_duplicated(String::from(left_max))
        && duplicated_number_max != duplicated_number_min
    {
        invalid_number += duplicated_number_max;
    }

    invalid_number
}

fn main() {
    let file_content = read_file("src/2.txt").unwrap();
    let binding = file_content.replace("\n", "");
    let file_content: Vec<&str> = binding.split(",").collect();
    let mut total = 0;

    for command in file_content {
        let left_right: Vec<&str> = command.split("-").collect();
        dbg!("{:?}", &left_right);
        let total_invalid =
            validate_invalid(String::from(left_right[0]), String::from(left_right[1]));

        dbg!("{:?}", total_invalid);

        total += total_invalid;
    }

    println!("Second challenge part 1: {:?}", total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_range_11_22() {
        let min = String::from("11");
        let max = String::from("22");

        let invalid = validate_invalid(min, max);

        assert_eq!(invalid, 33);
    }

    #[test]
    fn test_invalid_range_95_115() {
        let min = String::from("95");
        let max = String::from("115");

        let invalid = validate_invalid(min, max);

        assert_eq!(invalid, 99);
    }

    #[test]
    fn test_invalid_range_1188511880_1188511890() {
        let min = String::from("1188511880");
        let max = String::from("1188511890");

        let invalid = validate_invalid(min, max);

        assert_eq!(invalid, 1188511885);
    }

    #[test]
    fn test_invalid_range_2121212118_2121212124() {
        let min = String::from("2121212118");
        let max = String::from("2121212124");

        let invalid = validate_invalid(min, max);

        assert_eq!(invalid, 0);
    }

    #[test]
    fn test_invalid_range_998_1012() {
        let min = String::from("998");
        let max = String::from("1012");

        let invalid = validate_invalid(min, max);

        assert_eq!(invalid, 1010);
    }

    #[test]
    fn test_invalid_range_1_19() {
        let min = String::from("1");
        let max = String::from("19");

        let invalid = validate_invalid(min, max);

        assert_eq!(invalid, 11);
    }

    #[test]
    fn test_check_duplicated_true() {
        let val = String::from("2121221212");

        let invalid = check_duplicated(val);

        assert!(invalid);
    }

    #[test]
    fn test_check_duplicated_false() {
        let val = String::from("2121212118");

        let invalid = check_duplicated(val);

        assert!(!invalid);
    }
}
