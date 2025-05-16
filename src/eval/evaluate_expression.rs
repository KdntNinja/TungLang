use crate::interpreter::execute_block;
use crate::parser::Rule;
use crate::stdlib::StdLib;
use crate::value::{Value, Number, Float, StringValue, BooleanValue, Array, Dict};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use std::collections::HashMap;

pub fn evaluate_expression(
    pair: Pair<Rule>,
    variables: &Dict,
    stdlib: &StdLib,
) -> miette::Result<Value> {
    use crate::eval::operators::apply_operator;

    match pair.as_rule() {
        Rule::number => {
            let s: &str = pair.as_str();
            if s.contains('.') {
                Ok(Value::Float(Float(s.parse::<f64>().unwrap())))
            } else {
                Ok(Value::Number(Number(s.parse::<i64>().unwrap())))
            }
        }
        Rule::string => {
            let s: &str = pair.as_str();
            Ok(Value::String(StringValue(s[1..s.len() - 1].to_string())))
        }
        Rule::IDENTIFIER => {
            let name: &str = pair.as_str();
            match variables.get(name).cloned() {
                Some(val) => Ok(val),
                None => Err(miette::miette!(
                    "Error: Variable '{}' is not defined.",
                    name
                )),
            }
        }
        Rule::function_call => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let func_name: &str = inner.next().unwrap().as_str();
            let mut args: Vec<Value> = Vec::new();
            for p in inner {
                args.push(evaluate_expression(p, variables, stdlib)?);
            }
            if let Some(func) = stdlib.get(func_name) {
                let result = func(&args);
                Ok(result)
            } else if let Some(Value::Function { params, body, env }) = variables.get(func_name) {
                let mut local_vars: Dict = env.clone();
                for (i, param) in params.iter().enumerate() {
                    if let Some(arg) = args.get(i) {
                        local_vars.insert(param.clone(), arg.clone());
                    }
                }
                let parse_result = crate::parser::TungParser::parse(Rule::block, &body);
                match parse_result {
                    Ok(mut pairs) => {
                        let block = pairs.next().unwrap();
                        match execute_block(block, &mut local_vars, stdlib) {
                            Ok(_) => Ok(Value::Undefined),
                            Err(e) => {
                                let msg = e.to_string();
                                if msg.starts_with("__RETURN__") {
                                    let return_str = msg.trim_start_matches("__RETURN__").trim();
                                    let parse_result = crate::parser::TungParser::parse(Rule::expression, return_str);
                                    match parse_result {
                                        Ok(mut pairs) => {
                                            let expr = pairs.next().unwrap();
                                            let val = evaluate_expression(expr, &local_vars, stdlib)?;
                                            Ok(val)
                                        }
                                        Err(_) => Ok(Value::Undefined),
                                    }
                                } else {
                                    Err(e)
                                }
                            }
                        }
                    }
                    Err(e) => Err(miette::miette!("Function parse error: {}", e)),
                }
            } else {
                Err(miette::miette!(
                    "Error: Function '{}' is not defined.",
                    func_name
                ))
            }
        }
        Rule::comparison | Rule::sum | Rule::term => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let mut left = evaluate_expression(inner.next().unwrap(), variables, stdlib)?;
            while let Some(op_pair) = inner.next() {
                let op: &str = op_pair.as_str();
                let right = evaluate_expression(inner.next().unwrap(), variables, stdlib)?;
                left = apply_operator(left, right, op)?;
            }
            Ok(left)
        }
        Rule::factor => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let first: Pair<Rule> = inner.next().unwrap();
            evaluate_expression(first, variables, stdlib)
        }
        Rule::array => {
            let mut elements: Array = Vec::new();
            for p in pair.into_inner() {
                elements.push(evaluate_expression(p, variables, stdlib)?);
            }
            Ok(Value::Array(elements))
        }
        Rule::dict => {
            let mut map: Dict = Dict::new();
            for entry in pair.into_inner() {
                let mut kv: Pairs<Rule> = entry.into_inner();
                let k: String = kv.next().unwrap().as_str().to_string();
                let v: Value = evaluate_expression(kv.next().unwrap(), variables, stdlib)?;
                map.insert(k, v);
            }
            Ok(Value::Dict(map))
        }
        Rule::expression => {
            evaluate_expression(pair.into_inner().next().unwrap(), variables, stdlib)
        }
        _ => Err(miette::miette!("Error: Invalid expression.")),
    }
}
