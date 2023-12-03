fn main() {
    let file = include_str!("../../input2.txt");

     part2(file);
}


fn part2(input: &str) ->Result<i32,String> {
    let mut sum=0;

    let array_of_tuples:[(String,String);9]=[
    ("one".to_string(),"o1e".to_string()),
    ("two".to_string(),"t2o".to_string()),
    ("three".to_string(),"t3e".to_string()),
    ("four".to_string(),"f4r".to_string()),
    ("five".to_string(),"f5e".to_string()),
    ("six".to_string(),"s6x".to_string()),
    ("seven".to_string(),"s7n".to_string()),
    ("eight".to_string(),"e8t".to_string()),
    ("nine".to_string(),"n9e".to_string())
    ];
   
    for line in input.lines() {
        let mut modified_line = line.to_string();
        for tup in &array_of_tuples{
            modified_line= modified_line.replace(&tup.0, &tup.1);
        }

        let mut tempStr1=String::new();
        let mut tempStr2=String::new();
         
        modified_line.chars().for_each(|c|{
            if c.is_numeric(){
               tempStr1.push(c);
            }
       });

       if let Some(first_char) = tempStr1.chars().nth(0){
        tempStr2.push(first_char)
       }

       if let Some(last_char)= tempStr1.chars().last(){
        tempStr2.push(last_char);
       }

       match tempStr2.parse::<i32>() {
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