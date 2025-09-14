use std::io;

#[cfg(test)]

mod tests {

    use super::*;
     #[test]
     fn print_in_test(){
        let mut s = get_in();
        println!("{}", s);
     }
}


pub fn get_in() -> String{
    let mut flag = "ok";
    let mut string_to_get_in = String::new();
    let mut string_to_return = String::new();
    match io::stdin().read_line(&mut string_to_get_in){
        Ok(_) => string_to_return = string_to_get_in,
        Err(_) => flag = "er",
    }
    // TODO : Add a flag system which will alert the system if there is any problem
    return string_to_return;
}