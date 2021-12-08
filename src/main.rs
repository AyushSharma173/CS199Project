mod Parser;
use Parser::Parser as OtherParser;
use calculator_parser::calculate;

pub fn main(){
    //Parser::getinput(); 

    OtherParser::getOperator("10-10"); 
}
