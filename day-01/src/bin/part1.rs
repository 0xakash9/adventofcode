fn main() {
    let file = include_str!("../../input1.txt");
    let result = part1(file);
    println!("{:?}", result)
}

#[derive(Debug)]
struct Part1 {
    sum: i32,
}

impl Part1 {
    fn new() -> Self {
        Part1 { sum: 0 }
    }
    fn extract(&mut self, str: String) {
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

fn part1(input: &str) -> Result<i32, String> {
    let mut _data: Part1 = Part1::new();
    for line in input.lines() {
        let modified_line = line.to_string();

        _data.extract(modified_line);

        println!("{:?}", _data);
    }

    Ok(_data.sum)
}

#[cfg(test)]

mod tests {

    use super::*;
    #[test]
    fn test_process() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(142, part1(input).unwrap())
    }
}
