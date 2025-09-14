use std::{i32, io};

const SRS:&str = "There has been a Serious Error with Liberary of Input. Please Contact the Developers";

pub fn single_line() -> String{
    let mut _flag = "ok";
    let mut string_to_get_in = String::new();
    let mut string_to_return = String::new();
    match io::stdin().read_line(&mut string_to_get_in){
        Ok(_) => string_to_return = string_to_get_in,
        Err(_) => println!("Please Enter a Valid String"),
    }
    // TODO : Add a flag system which will alert the system if there is any problem
    return string_to_return;
}
pub fn number() -> i32{
    let mut _flag = "ok";
    let mut string_to_get_in = String::new();
    let mut string_to_return = String::new();
    match io::stdin().read_line(&mut string_to_get_in){
        Ok(_) => string_to_return = string_to_get_in,
        Err(_) => _flag = "er",
    }
    // Checking if it is a String one by one
    let word_length = string_to_return.len();
    let mut loop_variable = 0;
    while loop_variable > word_length {
        let char_to_check = string_to_return.chars().nth(loop_variable).expect(SRS);
        if char_to_check.is_ascii_digit() {
            println!("Please enter a Numerics only");
            return number();
        }        
    }
    //Conversion of string to int
    let int_to_return = string_to_return.trim().parse().unwrap();
    return int_to_return;
}