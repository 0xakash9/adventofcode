fn main() {
    let _input= include_str!("../input1.txt");
    part1(_input);
  }
  

  fn part1(input:&str){
  
    let mut _game_points_vec:Vec<u32> = Vec::new();  
    for input in input.lines(){
   
  
      let _parsed_str:Vec<&str> = input.split(":").collect();
      let card_str=_parsed_str[1];
    
     let _parsed_cards_str:Vec<&str> = card_str.split("|").collect();
    
     let cur_game_winning_card_numbers:Vec<&str> = _parsed_cards_str[0].split_whitespace().collect();
     let cur_game_card_numbers:Vec<&str> = _parsed_cards_str[1].split_whitespace().collect();
    
     let matching_pairs:Vec<&str> = cur_game_card_numbers.iter().filter(|&card_num|cur_game_winning_card_numbers.contains(card_num)).map(|&value| value).collect();
    
     let mut _points:u32=0;
    
     if matching_pairs.len() > 1{
        _points = u32::pow(2,  (matching_pairs.len() - 1) as u32); 
     }else if matching_pairs.len() == 1 {
         _points=1;
     }
    
  
     println!("{}",_points);
  
     if _points > 0{
         _game_points_vec.push(_points);
     }
    
    }  
  
  
    let _point_sum:u32= _game_points_vec.iter().sum();
    println!("{:#?}",_point_sum);
   
  }