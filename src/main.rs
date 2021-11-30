mod Parser;
use Parser::Parser as OtherParser;
// use calculator_parser::calculate;
use std::io::stdin;
use std::io::stdout;
use std::io::{self, Write};
 pub fn main(){
//     //Parser::getinput(); 


let mut s=String::new();
println!("Welcome to the clculator, please enter a calculation to be performed (with operators +,-,*,/,^) ");
let _= stdout().flush();
stdin().read_line(&mut s).expect("Did not enter a correct string");
println!("{}",OtherParser::parse_input(&s.to_string()))
;
 }
