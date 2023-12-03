fn main() {
    let file = include_str!("../../input1.txt");
    let result= part1(file);
    println!("{:?}",result)
}


fn part1(input: &str) ->Result<i32,String> {
    let mut sum=0;
    for line in input.lines() {
        let  modified_line = line.to_string();
       
        let mut temp_str1=String::new();
        let mut temp_str2=String::new();
         
        modified_line.chars().for_each(|c|{
            if c.is_numeric(){
               temp_str1.push(c);
            }
       });

       if let Some(first_char) = temp_str1.chars().nth(0){
        temp_str2.push(first_char)
       }

       if let Some(last_char)= temp_str1.chars().last(){
        temp_str2.push(last_char);
       }

       match temp_str2.parse::<i32>() {
            Ok(parsed_number) => {
                sum+=parsed_number
            }
            Err(_) => {
                println!("Failed to parse the string as an integer");
            }
      }
    }

    println!("{}",sum);
    Ok(sum)
}