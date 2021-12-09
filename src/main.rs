mod Parser;
use Parser::Parser as OtherParser;
// use calculator_parser::calculate;

 pub fn main(){
//     //Parser::getinput(); 

    println!("{} ",OtherParser::parse_input("100*100+100+100")); 

    let mut vec : Vec<f32> = Vec::new();
    vec.push(100.0);
    vec.push(100.0);
    vec.push(105.5); 
    
    OtherParser::write_to_csv(vec); 
 }
