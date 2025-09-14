use std::io;

pub fn single_line() -> String{
    let mut _flag = "ok";
    let mut string_to_get_in = String::new();
    let mut string_to_return = String::new();
    match io::stdin().read_line(&mut string_to_get_in){
        Ok(_) => string_to_return = string_to_get_in,
        Err(_) => _flag = "er",
    }
    // TODO : Add a flag system which will alert the system if there is any problem
    return string_to_return;
}
