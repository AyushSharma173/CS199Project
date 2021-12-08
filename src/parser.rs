 use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{char, digit1, space0},
    combinator::recognize,
    multi::many0,
    sequence::delimited,
    IResult,
    number::complete::f32
};

pub enum Operator{
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Root
}

pub enum Parser{
    num (f32),
    operator (Operator)
}

impl Parser{

    pub fn get_input(input: &str) //-> Result<(&str,()),&str>
    {
        // let mut number1 = 0 as f32;
        // let mut number2 = 0 as f32; 
        // let mut oper = Operator::Add; 

        // //number1 = many0(f32); 
        // //let oper_str = tag("+");
        // oper = Self::getOperator("+"); 
        // //number2 = many0(f32); 

        // return (number1,number2,oper)

    }


    pub fn getOperator(input:&str) -> Operator{
        let mut operation = Operator::Add; 
        for i in input.chars(){
        match i{
            '+' => operation = Operator::Add,
            '-' => operation = Operator::Subtract,
            '*' => operation = Operator::Multiply,
            '/' => operation = Operator::Divide,
            '^' => operation = Operator::Power,
            _ => continue,
        }
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
            Operator::Power => result = num1.powf(num2),
            Operator::Root => result = num1.powf(1 as f32/num2)
        }
        return result; 
    }
}