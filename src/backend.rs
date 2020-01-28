use crate::parser::AST;

fn evaluate_function_call(id: &String, params: &Vec<AST>) -> Result<Option<f64>, String> {
    Ok(match &id[..] {
        "sin" => {
            if params.len() != 1 {
                None
            } else {
                Some(evaluate(&params[0])?.sin())
            }
        }
        "cos" => {
            if params.len() != 1 {
                None
            } else {
                Some(evaluate(&params[0])?.cos())
            }
        }
        "pow" => {
            if params.len() != 2 {
                None
            } else {
                Some(evaluate(&params[0])?.powf(evaluate(&params[1])?))
            }
        }
        "ln" => {
            if params.len() != 1 {
                None
            } else {
                Some(evaluate(&params[0])?.ln())
            }
        }
        _ => None,
    })
}

fn evaluate_global_constant(id: &String) -> Option<f64> {
    match &id[..] {
        "e" => Some(std::f64::consts::E),
        "pi" => Some(std::f64::consts::PI),
        _ => None,
    }
}

pub fn evaluate(tree: &AST) -> Result<f64, String> {
    match tree {
        AST::Add(a, b) => Ok(evaluate(&a)? + evaluate(&b)?),
        AST::Subtract(a, b) => Ok(evaluate(&a)? - evaluate(&b)?),
        AST::Multiply(a, b) => Ok(evaluate(&a)? * evaluate(&b)?),
        AST::Divide(a, b) => Ok(evaluate(&a)? / evaluate(&b)?),
        AST::Negate(x) => Ok(-evaluate(&x)?),
        AST::Number(n) => Ok(*n),
        AST::Identifier(id) => evaluate_global_constant(id).ok_or(format!(
            "Invalid identifier, global constant `{}` does not exist",
            id
        )),
        AST::Call(id, params) => evaluate_function_call(id, params)?.ok_or(format!(
            "Invalid function call, function `{}/{}` does not exist",
            id,
            params.len()
        )),
        AST::Let(name, expr) => {
            let val = evaluate(&expr)?;
            println!("{} = {}", name, val);
            Ok(val)
        }
    }
}
