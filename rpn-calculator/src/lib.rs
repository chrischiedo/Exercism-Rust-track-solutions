#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = Vec::<i32>::new();

    for elem in inputs {
        match elem {
            CalculatorInput::Value(number) => stack.push(*number),
            _ => {
                let v2 = stack.pop();
                let v1 = stack.pop();
                match (elem, v1, v2) {
                    (CalculatorInput::Add, Some(v1), Some(v2)) => stack.push(v1 + v2),
                    (CalculatorInput::Subtract, Some(v1), Some(v2)) => stack.push(v1 - v2),
                    (CalculatorInput::Divide, Some(v1), Some(v2)) => stack.push(v1 / v2),
                    (CalculatorInput::Multiply, Some(v1), Some(v2)) => stack.push(v1 * v2),
                    _ => return None
                }
            }
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
