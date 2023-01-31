use std::vec;

#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn calculate(x:i32, y:i32, operation: &CalculatorInput) -> i32{
    let result = match operation{
        &CalculatorInput::Add => x + y,
        &CalculatorInput::Subtract => x - y,
        &CalculatorInput::Multiply => x * y,
        &CalculatorInput::Divide => x / y,
        _ => panic!()
    };
    result
}


pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack:Vec<i32> = vec![];
    for input in inputs {
        match input {
            &CalculatorInput::Value(x) => stack.push(x),
            _ => {
                let c1 = stack.pop();
                let c2 = stack.pop();
                match (c1, c2, input){
                    (Some(y), Some(x), operation) => {
                        stack.push(calculate(x, y, operation))
                    }
                    _ => return None
                }
            }
        }
    }
    match stack.len() {
        1 => stack.pop(),
        _ => None
    }
}
