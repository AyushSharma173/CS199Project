 use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{char, digit1, space0},
    combinator::recognize,
    multi::many0,
    sequence::delimited,
    IResult
};

pub enum Operator{
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Root
}

pub struct equation{
    num1: f32
    operation: Operator
    num2: f32
}

impl equation{

    fn getinput(input: &str) -> IResult<f32, f32, Operator>{
        let mut number1 = 0 as f32;
        let mut number2 = 0 as f32; 
        let mut oper = ""; 

        number1 = many0(f32); 
        oper = tag("+"||"-"||"*"||"/"||"^"||"root"); 

        Ok(number1,number2,getOperator(oper))
    }


    fn getOperator(input:&str) -> Operator{
        match input{
            "+" => operation = Operator::Add,
            "-" => operation = Operator::Subtract,
            "*" => operation = Operator::Multiply,
            "/" => operation = Operator::Divide,
            "^" => operation = Operator::Power,
            "root" => operation = Operator::Root
        }
        return operation; 
    }

    fn doEquation(num1:f32,num2:f32,operation:Operator)->f32{
        let mut result = 0 as f32; 
        match operation{
            Operator::Add => result = num1 + num2,
            Operator::Subtract => result = num1 - num2,
            Operator::Multiply => result = num1*num2,
            Operator::Divide => result = num1/num2,
            Operator::Power => result = num1.pow(num2),
            Operator::Sqrt => result = num1.nth_root(num2)
        }
        return result; 
    }
}