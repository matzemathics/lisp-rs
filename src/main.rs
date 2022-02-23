extern crate pest;
#[macro_use]
extern crate pest_derive;

mod ast;
mod bytecode;
mod injective;
use crate::ast::LispValue;
use crate::bytecode::{AstValue, Bytecode, Compiled, Entry, Heap};

use std::collections::HashMap;
use std::convert::{TryInto, From};
use std::ops::Deref;

#[derive(Debug, Clone, Default)]
pub struct LispRecord {
    val: LispValue,
    properties: HashMap<String, LispValue>,
}

impl PartialEq for LispRecord {
    fn eq(&self, v: &LispRecord) -> bool {
        if self.val.get_type() != v.val.get_type() {
            return false;
        }
        unimplemented!()
    }
}

injective_from_property!(bool => LispRecord, val, LispValue);
injective_from_property!(String => LispRecord, val, LispValue);
injective_from_property!(char => LispRecord, val, LispValue);
injective_from_property!(f64 => LispRecord, val, LispValue);
injective_from_property!(LispValue => LispRecord, val, LispValue);

#[derive(Debug)]
pub struct LispHeap(HashMap<String, LispRecord>);

impl Heap<String, LispRecord> for LispHeap {
    fn insert(&mut self, symbol: String, value: LispRecord) {
        let mut new_value = value;
        if let Some(old_value) = self.0.get(&symbol) {
            for prop in old_value.properties.keys() {
                if !new_value.properties.contains_key(prop) {
                    new_value.properties.insert(
                        prop.clone(),
                        old_value.properties.get(prop).cloned().unwrap(),
                    );
                }
            }
        }
        self.0.insert(symbol, new_value);
    }
    fn get(&self, symbol: &String) -> Option<LispRecord> {
        self.0.get(symbol).cloned()
    }
}

impl Entry for String {
    fn entry() -> String {
        String::from("entry")
    }
}

impl AstValue for LispValue {
    type OutputValue = LispRecord;
    type SymbolValue = String;

    fn compile(&self) -> Compiled<Self::SymbolValue, Self::OutputValue> {
        let mut res = Compiled::new();
        
        if self.literal() {
            res.append(Bytecode::PushConst(self.clone().into()))
        } 
        else if let LispValue::Expression(e) = self {
            let inner: LispValue = e.deref().clone();
            res.append(Bytecode::PushConst(inner.into()))
        } 
        else if let LispValue::Symbol(s) = self {
            res.append(Bytecode::Push(s.clone()));
        }
        else if let LispValue::List(v) = self {
            let mut elements = v.iter();
            
            if let Some(func) = elements.next() {
                while let Some(arg) = elements.next() {
                    res.append_compiled(arg.compile());
                }
    
                if let LispValue::Symbol(s) = func {
                    res.append(Bytecode::Call(s.clone()));
                } else {
                    
                    
                }
            }
        }
        res
    }
}

impl AstValue for Vec<LispValue> {
    type OutputValue = LispRecord;
    type SymbolValue = String;
    fn compile(&self) -> Compiled<Self::SymbolValue, Self::OutputValue> {
        let mut res = Compiled::new();
        let mut entry = Vec::new();
        for cmd in self {
            let stmts = cmd.compile();
            stmts.entry().iter().for_each(|x| entry.push(x.clone()));
        }
        res.add_entry(String::entry(), entry);
        res
    }
}

fn main() {
    let input = "(+ 12 1)";

    println!("Parsed {:?}", ast::parse(input).compile());
}
