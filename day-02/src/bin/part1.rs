use std::i32;

use regex::Regex;

fn main() {
    let input = include_str!("../../input1.txt");
    part1(input);
}


fn part1(input: &str) {
    let mut _games: Vec<&str> = Vec::new();

  
    for ipt in input.lines() {
        let _slices: Vec<&str> = ipt.split(":").collect();
        let pattern = Regex::new(r"[,;]\s*").unwrap();
        let comb_coby: Vec<&str> = pattern.split(_slices[1]).collect();

        let mut _is_valid = Vec::new();
        let _is_valid_comb = comb_coby.iter().for_each(|comb| {
            let parsed_str: Vec<&str> = comb.split_whitespace().collect();
            let num_str: &str = parsed_str[0];
            let color_str = parsed_str[1];
            let _num: i32 = match num_str.parse() {
                Ok(number) => number,
                Err(_) => 0,
            };

            
            if color_str ==  "red" && _num > 12 {
                _is_valid.push(_num)
            } else if color_str == "blue" && _num > 14 {
                _is_valid.push(_num)
            } else if color_str == "green" && _num > 13 {
                _is_valid.push(_num)
            }
        });

        if _is_valid.len() == 0 {
            let sliced_str: Vec<&str> = _slices[0].split_whitespace().collect();
            _games.push(sliced_str[1])
        }
    }

    let parsed_game_id: Vec<i32> = _games
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    let _sm: i32 = parsed_game_id.iter().sum();

    println!("{}", _sm)
}
