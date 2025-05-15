use crate::eval::evaluate_expression::evaluate_expression;
use crate::parser::Rule;
use crate::stdlib::StdLib;
use crate::value::Value;
use pest::iterators::{Pair, Pairs};
use std::collections::HashMap;

pub fn run_program(parsed: Pairs<Rule>) -> miette::Result<()> {
    let mut variables: HashMap<String, Value> = HashMap::new();
    let stdlib: StdLib = StdLib::new();
    for pair in parsed {
        execute_statement(pair, &mut variables, &stdlib)?;
    }
    Ok(())
}

fn execute_statement(
    pair: Pair<Rule>,
    variables: &mut HashMap<String, Value>,
    stdlib: &StdLib,
) -> miette::Result<()> {
    match pair.as_rule() {
        Rule::variable_declaration => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let var_name: String = inner.next().unwrap().as_str().to_string();
            let value: Value = evaluate_expression(inner.next().unwrap(), variables, stdlib)?;
            variables.insert(var_name, value);
        }
        Rule::assignment => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let var_name: String = inner.next().unwrap().as_str().to_string();
            let value: Value = evaluate_expression(inner.next().unwrap(), variables, stdlib)?;
            if variables.contains_key(&var_name) {
                variables.insert(var_name, value);
            } else {
                return Err(miette::miette!(
                    "Assignment to undefined variable '{}'.",
                    var_name
                ));
            }
        }
        Rule::augmented_assignment => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let var_name: String = inner.next().unwrap().as_str().to_string();
            let op_pair: Pair<Rule> = inner.next().unwrap();
            let op: &str = op_pair.as_str();
            let value: Value = evaluate_expression(inner.next().unwrap(), variables, stdlib)?;
            if let Some(current) = variables.get(&var_name).cloned() {
                let op_str: &str = &op[..op.len() - 1];
                let new_value: Value =
                    crate::eval::operators::apply_operator(current, value, op_str)?;
                variables.insert(var_name, new_value);
            } else {
                return Err(miette::miette!(
                    "Assignment to undefined variable '{}'.",
                    var_name
                ));
            }
        }
        Rule::print_statement => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let value: Value = evaluate_expression(inner.next().unwrap(), variables, stdlib)?;
            match value {
                Value::String(s) => println!("{}", s),
                Value::Number(n) => println!("{}", n),
                Value::Float(f) => println!("{}", f),
                Value::Boolean(b) => println!("{}", b),
                Value::Array(arr) => println!("{:?}", arr),
                Value::Dict(map) => println!("{:?}", map),
                Value::Function { .. } => println!("<function>"),
                Value::Undefined => {
                    return Err(miette::miette!("Attempted to print an undefined value."))
                }
            }
        }
        Rule::if_statement => {
            execute_if_statement(pair, variables, stdlib)?;
        }
        Rule::while_statement => {
            let mut inner: Pairs<Rule> = pair.into_inner();
            let condition: Pair<Rule> = inner.next().unwrap();
            let block: Pair<Rule> = inner.next().unwrap();
            loop {
                let cond_val = evaluate_expression(condition.clone(), variables, stdlib)?;
                if !is_truthy(cond_val.clone()) {
                    break;
                }
                let mut local_vars: HashMap<String, Value> = variables.clone();
                execute_block(block.clone(), &mut local_vars, stdlib)?;
                for (k, v) in local_vars.iter() {
                    if variables.contains_key(k) {
                        variables.insert(k.clone(), v.clone());
                    }
                }
            }
        }
        Rule::function_definition => {
            let mut inner = pair.into_inner();
            let fn_name = inner.next().unwrap().as_str().to_string();
            let mut params = Vec::new();
            let mut body = None;
            if let Some(next) = inner.next() {
                if next.as_rule() == Rule::IDENTIFIER {
                    // At least one parameter
                    params.push(next.as_str().to_string());
                    for p in next.into_inner() {
                        params.push(p.as_str().to_string());
                    }
                    // The next item must be the body
                    if let Some(b) = inner.next() {
                        body = Some(b.as_str().to_string());
                    }
                } else if next.as_rule() == Rule::block {
                    // No parameters, next is body
                    body = Some(next.as_str().to_string());
                }
            }
            if let Some(body_str) = body {
                variables.insert(
                    fn_name,
                    Value::Function {
                        params,
                        body: body_str,
                        env: variables.clone(),
                    },
                );
            }
        }
        Rule::return_statement => {
            let value = evaluate_expression(pair.into_inner().next().unwrap(), variables, stdlib)?;
            return Err(miette::miette!("__RETURN__{:?}", value));
        }
        _ => {}
    }
    Ok(())
}

fn is_truthy(value: Value) -> bool {
    match value {
        Value::Number(n) => n != 0,
        Value::Float(f) => f != 0.0,
        Value::String(ref s) => !s.is_empty(),
        Value::Boolean(b) => b,
        Value::Array(ref arr) => !arr.is_empty(),
        Value::Dict(ref map) => !map.is_empty(),
        Value::Undefined => false,
        Value::Function { .. } => false,
    }
}

fn execute_if_statement(
    pair: Pair<Rule>,
    variables: &mut HashMap<String, Value>,
    stdlib: &StdLib,
) -> miette::Result<()> {
    let mut inner: Pairs<Rule> = pair.into_inner();
    let condition: Pair<Rule> = inner.next().unwrap();
    let block: Pair<Rule> = inner.next().unwrap();
    let cond_val = evaluate_expression(condition, variables, stdlib)?;
    let condition_met: bool = is_truthy(cond_val);
    if condition_met {
        let mut local_vars: HashMap<String, Value> = variables.clone();
        execute_block(block, &mut local_vars, stdlib)?;
        for (k, v) in local_vars.iter() {
            if variables.contains_key(k) {
                variables.insert(k.clone(), v.clone());
            }
        }
        return Ok(());
    } else {
        for elif_or_else in inner {
            match elif_or_else.as_rule() {
                Rule::elif_block => {
                    let mut elif_inner: Pairs<Rule> = elif_or_else.into_inner();
                    let elif_condition: Pair<Rule> = elif_inner.next().unwrap();
                    let elif_block: Pair<Rule> = elif_inner.next().unwrap();
                    let elif_val = evaluate_expression(elif_condition, variables, stdlib)?;
                    let elif_met: bool = is_truthy(elif_val);
                    if elif_met {
                        let mut local_vars: HashMap<String, Value> = variables.clone();
                        execute_block(elif_block, &mut local_vars, stdlib)?;
                        for (k, v) in local_vars.iter() {
                            if variables.contains_key(k) {
                                variables.insert(k.clone(), v.clone());
                            }
                        }
                        return Ok(());
                    }
                }
                Rule::else_block => {
                    let else_block: Pair<Rule> = elif_or_else.into_inner().next().unwrap();
                    let mut local_vars: HashMap<String, Value> = variables.clone();
                    execute_block(else_block, &mut local_vars, stdlib)?;
                    for (k, v) in local_vars.iter() {
                        if variables.contains_key(k) {
                            variables.insert(k.clone(), v.clone());
                        }
                    }
                    return Ok(());
                }
                _ => {}
            }
        }
    }
    Ok(())
}

pub fn execute_block(
    block: Pair<Rule>,
    variables: &mut HashMap<String, Value>,
    stdlib: &StdLib,
) -> miette::Result<()> {
    let mut local_vars: HashMap<String, Value> = variables.clone();
    for statement in block.into_inner() {
        execute_statement(statement, &mut local_vars, stdlib)?;
    }
    for (k, v) in local_vars.iter() {
        if variables.contains_key(k) {
            variables.insert(k.clone(), v.clone());
        }
    }
    Ok(())
}
