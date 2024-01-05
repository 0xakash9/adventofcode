fn main() {
    let file = include_str!("../../input2.txt");

    let _ = part2(file);
}

struct Part2 {
    sum: i32,
}

impl Part2 {
    fn new() -> Self {
        Part2 { sum: 0 }
    }
    fn extract(&mut self, mut str: String) {
        let array_of_tuples: [(String, String); 9] = [
            ("one".to_string(), "o1e".to_string()),
            ("two".to_string(), "t2o".to_string()),
            ("three".to_string(), "t3e".to_string()),
            ("four".to_string(), "f4r".to_string()),
            ("five".to_string(), "f5e".to_string()),
            ("six".to_string(), "s6x".to_string()),
            ("seven".to_string(), "s7n".to_string()),
            ("eight".to_string(), "e8t".to_string()),
            ("nine".to_string(), "n9e".to_string()),
        ];
        for tup in &array_of_tuples {
            str = str.replace(&tup.0, &tup.1);
        }

        let temp_str: Vec<char> = str.chars().filter(|e| e.is_numeric()).collect();

        println!("{:?}", temp_str);
        let temp_str2: String = temp_str[0].to_string() + &temp_str[temp_str.len() - 1].to_string();

        match temp_str2.parse::<i32>() {
            Ok(parsed_number) => self.sum += parsed_number,
            Err(_) => {
                println!("Failed to parse the string as an integer");
            }
        }
    }
}
fn part2(input: &str) -> Result<i32, String> {
    let mut _data: Part2 = Part2::new();
    for line in input.lines() {
        let modified_line = line.to_string();

        _data.extract(modified_line);
    }

    Ok(_data.sum)
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn process() {
        let input = "two1nine
                eightwothree
                abcone2threexyz
                xtwone3four
                4nineeightseven2
                zoneight234
                7pqrstsixteen";
        assert_eq!(281, part2(input).unwrap());
    }
}
