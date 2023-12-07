use regex::Regex;
use std::collections::HashMap;

fn main(){
    let input= include_str!("../../input2.txt");
     part2(input);
}
 
fn part2(input:&str){

    let mut _power_of_cubes_vector:Vec<i32> = Vec::new();

    for ipt in input.lines(){
        let _slices:Vec<&str> = ipt.split(":").collect();
        let pattern = Regex::new(r"[,;]\s*").unwrap();
        let comb_coby: Vec<&str> = pattern.split(_slices[1]).collect();

        let mut max_cube_map: HashMap<String, i32> = HashMap::new();

        let _is_valid_comb= comb_coby.iter().for_each(|comb|{
            let parsed_str:Vec<&str> =  comb.split_whitespace().collect();
            let num_str:&str = parsed_str[0];
            let color_str:&str  = parsed_str[1];
            let _num: i32 = match num_str.parse() {
              Ok(number) => number,
              Err(_) => 0,
          };
  
            if let  Some(value) = max_cube_map.get(color_str) {
                if *value < _num {
                max_cube_map.insert(color_str.to_string(), _num);
                }
            }else {
                max_cube_map.insert(color_str.to_string(), _num);
            }
        });

        
       let _max_possible_combinations:Vec<i32> = max_cube_map.into_values().collect();
       let _power_of_cubes:i32=_max_possible_combinations.iter().product();

       _power_of_cubes_vector.push(_power_of_cubes);
    }

    let _sum_of_powers:i32 = _power_of_cubes_vector.iter().sum();

     println!("{}",_sum_of_powers)
}