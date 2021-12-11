use std::{
    env::args,
};
    
fn main(){

    const DNA: &str = "\
    O       o O       o O       o \n\
    | O   o | | O   o | | O   o | \n\
    | | O | | | | O | | | | O | | \n\
    | o   O | | o   O | | o   O | \n\
    o       O o       O o       O \n\
    ";

    let args: Vec<String> = args().collect();
    let mut input_str: &str = &args[1];

    let strlen = input_str.len();

    if strlen > 75 {
        input_str = "Brevity is the soul of wit!";
    }

    let top_bubble: String = vec!['-'; strlen+2].iter().collect();
    let mid_bubble: String = String::from("(") + &input_str + &String::from(")");
    let end_bubble: String = vec!['-'; strlen+2].iter().collect();
    let mut line_line: String = vec![' '; strlen/2].iter().collect();
    line_line += "|";
    
    // Print ART
    println!("{}", top_bubble);
    println!("{}", mid_bubble);
    println!("{}", end_bubble);
    println!("{}", line_line);
    println!("{}", line_line);
    println!("{}", DNA);
}