
#[derive(Debug)]
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

    pub fn parse_input(input: &str) -> f32
    {
         
            let mut vec: Vec<String> = Self::get_substr(input).0; 
            let mut oper: Vec<Operator> = Self::get_substr(input).1;

            let mut final_result = Self::doEquation(vec[0].parse::<f32>().unwrap(),vec[1].parse::<f32>().unwrap(),&oper[0]);
            
            vec.remove(0);
            vec.remove(1); 
            oper.remove(0); 

            let mut idx = 0; 

            while !(vec.is_empty()){
                final_result = Self::doEquation(final_result,vec[0].parse::<f32>().unwrap(),&oper[0]);
                vec.remove(0);
                oper.remove(0); 
            }

            return final_result; 
    }

    //Adapted from a provided function in CS128 MP2
    pub fn get_substr(input: &str) ->(Vec<String>,Vec<Operator>){
        //let mut last = 0; 
        let mut vec: Vec<String> = Vec::new();
        let mut operators: Vec<Operator> = Vec::new(); 

        let mut curr_idx = 0; 
        let mut len = 0; 

        for i in input.chars(){
            if (i == '+')||(i == '-')||(i == '*')||(i == '/')||(i == '^'){
                operators.push(Self::getOperator(i)); 

                let mut temp = String::new();
             
                temp = input.chars().skip(curr_idx).take(len).collect();   
                curr_idx += len+1; 
                len = 0; 
                vec.push(temp); 
            }
            else{
            len+=1; 
            if curr_idx+len == input.chars().count()-1{
                let mut temp = String::new(); 
                temp = input.chars().skip(curr_idx).take(len+1).collect();
                vec.push(temp); 
            }
            } 
        }

        return (vec,operators); 

    }


    pub fn getOperator(input:char) -> Operator{
        let mut operation = Operator::Add; 
        //for i in input.chars(){
        match input{
            '+' => operation = Operator::Add,
            '-' => operation = Operator::Subtract,
            '*' => operation = Operator::Multiply,
            '/' => operation = Operator::Divide,
            '^' => operation = Operator::Power,
            _ => unreachable!(),
        }
    //}
        return operation; 
    }


    pub fn doEquation(num1:f32,num2:f32,operation:&Operator)->f32{
        let mut result; 
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