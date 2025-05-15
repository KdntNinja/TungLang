use crate::parser::Rule;
use crate::stdlib::StdLib;
use crate::value::Value;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub fn evaluate_expression(
    pair: Pair<Rule>,
    variables: &HashMap<String, Value>,
    stdlib: &StdLib,
) -> miette::Result<Value> {
    use crate::eval::operators::apply_operator;

    match pair.as_rule() {
        Rule::number => {
            let s: &str = pair.as_str();
            if s.contains('.') {
                Ok(Value::Float(s.parse::<f64>().unwrap()))
            } else {
                Ok(Value::Number(s.parse::<i64>().unwrap()))
            }
        }
        Rule::string => {
            let s: &str = pair.as_str();
            Ok(Value::String(s[1..s.len() - 1].to_string()))
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
            let mut args = Vec::new();
            for p in inner {
                args.push(evaluate_expression(p, variables, stdlib)?);
            }
            if let Some(func) = stdlib.get(func_name) {
                let result = func(&args);
                Ok(result)
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
            let mut elements = Vec::new();
            for p in pair.into_inner() {
                elements.push(evaluate_expression(p, variables, stdlib)?);
            }
            Ok(Value::Array(elements))
        }
        Rule::dict => {
            let mut map: HashMap<String, Value> = HashMap::new();
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
