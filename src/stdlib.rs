// TungLang standard library setup and function dispatch
use crate::eval::std::std_abs::std_abs;
use crate::eval::std::std_cast;
use crate::eval::std::std_input::tunglang_input;
use crate::eval::std::std_len::std_len;
use crate::eval::std::std_list;
use crate::eval::std::std_math;
use crate::eval::std::std_print::std_print;
use crate::eval::std::std_range;
use crate::value::{
    Array, BooleanValue, BuiltinFn, Dict, FloatNumber, Integer, StringValue, Value,
};
use std::collections::HashMap;

pub struct StdLib {
    functions: HashMap<&'static str, BuiltinFn>,
}

impl StdLib {
    pub fn new() -> Self {
        let mut functions: HashMap<&'static str, BuiltinFn> = HashMap::new();

        // Basic functions
        functions.insert("input", |args: &[Value]| {
            tunglang_input(
                args.get(0)
                    .unwrap_or(&Value::String(StringValue::new(String::new()))),
            )
        });
        functions.insert("print", |args: &[Value]| {
            std_print(
                args.get(0)
                    .unwrap_or(&Value::String(StringValue::new(String::new()))),
            );
            Value::Integer(Integer::new(0))
        });
        functions.insert("abs", |args: &[Value]| {
            std_abs(args.get(0).unwrap_or(&Value::Integer(Integer::new(0))))
        });
        functions.insert("len", |args: &[Value]| {
            std_len(
                args.get(0)
                    .unwrap_or(&Value::String(StringValue::new(String::new()))),
            )
        });
        functions.insert("range", std_range::std_range);

        // Type conversion functions (like Python)
        functions.insert("int", |args: &[Value]| {
            std_cast::std_int(args.get(0).unwrap_or(&Value::Integer(Integer::new(0))))
        });
        functions.insert("str", |args: &[Value]| {
            std_cast::std_str(
                args.get(0)
                    .unwrap_or(&Value::String(StringValue::new(String::new()))),
            )
        });
        functions.insert("float", |args: &[Value]| {
            std_cast::std_float(args.get(0).unwrap_or(&Value::Integer(Integer::new(0))))
        });
        functions.insert("bool", |args: &[Value]| {
            std_cast::std_bool(args.get(0).unwrap_or(&Value::Integer(Integer::new(0))))
        });

        // Math functions (like Python)
        functions.insert("min", std_math::std_min);
        functions.insert("max", std_math::std_max);
        functions.insert("sum", std_math::std_sum);
        functions.insert("round", std_math::std_round);

        // List functions (like Python)
        functions.insert("append", std_list::std_append);
        functions.insert("insert", std_list::std_insert);
        functions.insert("pop", std_list::std_pop);
        functions.insert("index", std_list::std_index);
        functions.insert("sort", std_list::std_sort);

        Self { functions }
    }

    pub fn get(&self, name: &str) -> Option<&BuiltinFn> {
        self.functions.get(name)
    }
}
