#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

fn do_arithmetic(stack: &mut Vec<i32>, operation: &CalculatorInput) -> Result<(), ()> {
    let first = stack.pop();
    let second = stack.pop();
    match (first, second) {
        (None, _) | (_, None) => return Err(()),
        (Some(a), Some(b)) => match operation {
            CalculatorInput::Add => stack.push(b + a),
            CalculatorInput::Subtract => stack.push(b - a),
            CalculatorInput::Multiply => stack.push(b * a),
            CalculatorInput::Divide => stack.push(b / a),
            _ => panic!("Not possible"),
        },
    };
    Ok(())
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();
    for c in inputs {
        match c {
            CalculatorInput::Add
            | CalculatorInput::Subtract
            | CalculatorInput::Multiply
            | CalculatorInput::Divide => do_arithmetic(&mut stack, c).ok()?,
            CalculatorInput::Value(v) => stack.push(*v),
        }
    }
    if stack.len() == 1 {
        stack.first().copied()
    } else {
        None
    }
}
