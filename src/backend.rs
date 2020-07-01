use crate::parser::AST;

use std::collections::HashMap;

fn evaluate_function_call(id: &String, params: &Vec<AST>, variables: &mut HashMap<String, f64>) -> Result<Option<f64>, String> {
    Ok(match &id[..] {
        "sin" => {
            if params.len() != 1 {
                None
            } else {
                Some(evaluate(&params[0], variables)?.sin())
            }
        }
        "cos" => {
            if params.len() != 1 {
                None
            } else {
                Some(evaluate(&params[0], variables)?.cos())
            }
        }
        "pow" => {
            if params.len() != 2 {
                None
            } else {
                Some(evaluate(&params[0], variables)?.powf(evaluate(&params[1], variables)?))
            }
        }
        "ln" => {
            if params.len() != 1 {
                None
            } else {
                Some(evaluate(&params[0], variables)?.ln())
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

pub fn evaluate(tree: &AST, variables: &mut HashMap<String, f64>) -> Result<f64, String> {
    match tree {
        AST::Add(a, b) => Ok(evaluate(&a, variables)? + evaluate(&b, variables)?),
        AST::Subtract(a, b) => Ok(evaluate(&a, variables)? - evaluate(&b, variables)?),
        AST::Multiply(a, b) => Ok(evaluate(&a, variables)? * evaluate(&b, variables)?),
        AST::Divide(a, b) => Ok(evaluate(&a, variables)? / evaluate(&b, variables)?),
        AST::Negate(x) => Ok(-evaluate(&x, variables)?),
        AST::Number(n) => Ok(*n),
        AST::Identifier(id) => evaluate_global_constant(id).or(variables.get(id).copied()).ok_or(format!(
            "Invalid identifier, global constant `{}` does not exist",
            id
        )),
        AST::Call(id, params) => evaluate_function_call(id, params, variables)?.ok_or(format!(
            "Invalid function call, function `{}/{}` does not exist",
            id,
            params.len()
        )),
        AST::Let(name, expr) => {
            let val = evaluate(&expr, variables)?;
            variables.insert(name.clone(), val);
            Ok(val)
        }
        AST::Chain(chain) => {
            let mut val = 0f64;
            for statement in chain {
                val = evaluate(statement, variables)?;
            }
            Ok(val)
        }
    }
}
