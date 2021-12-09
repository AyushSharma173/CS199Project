mod Parser;
use Parser::Parser as OtherParser;
// use calculator_parser::calculate;

 pub fn main(){
//     //Parser::getinput(); 

    println!("{} ",OtherParser::parse_input("100+100+100")); 

    
 }
