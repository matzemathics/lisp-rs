use pest::iterators::Pair;
use pest::Parser;

use std::convert::TryFrom;
use std::convert::TryInto;
use std::iter::FromIterator;
use std::u32;

use crate::injective_from_pattern;
use crate::injective::ConversionError;

#[derive(Parser)]
#[grammar = "lisp.pest"]
pub struct LispParser;

#[derive(Debug, Clone)]
pub enum LispValue {
    List(Vec<LispValue>),
    Number(f64),
    String(String),
    Symbol(String),
    Char(char),
    Expression(Box<LispValue>),
}

pub fn parse(input: &str) -> Vec<LispValue> {
    let parsed = LispParser::parse(Rule::expr, input).unwrap_or_else(|e| panic!("{}", e));

    let mut asts = Vec::new();
    for expr in parsed {
        asts.push(build_ast(expr))
    }

    asts
}

fn build_ast(parsed: Pair<Rule>) -> LispValue {
    match parsed.as_rule() {
        Rule::expr => {
            // build_ast(parsed.into_inner().next().unwrap())
            panic!("Didn't expect symbol in parse-tree")
        }
        Rule::list => {
            let mut vec = Vec::new();
            for item in parsed.into_inner() {
                vec.push(build_ast(item));
            }
            LispValue::List(vec)
        }
        Rule::symbol => {
            let s = String::from(parsed.as_str());
            LispValue::Symbol(s)
        }
        Rule::number => {
            let n = String::from(parsed.as_str());
            LispValue::Number(n.parse().unwrap())
        }
        Rule::string => LispValue::String(parse_str(parsed.as_span())),
        Rule::char => {
            let mut chr_str = parse_str(parsed.as_span());
            assert_eq!(chr_str.pop(), Some('\''));
            LispValue::Char(chr_str.pop().unwrap())
        }
        Rule::value_expr => {
            let inner = parsed.into_inner().next().unwrap();
            LispValue::Expression(Box::from(build_ast(inner)))
        }
        _ => unimplemented!(),
    }
}

fn parse_str(span: pest::Span) -> String {
    let mut s = String::new();
    let mut input = span.as_str().chars();

    while let Some(c) = input.next() {
        if c != '\\' {
            s.push(c)
        } else {
            s.push(match input.next() {
                Some('n') => '\n',
                Some('r') => '\r',
                Some('t') => '\t',
                Some('\\') => '\\',
                Some('0') => '0',
                Some('u') => {
                    assert_eq!(input.next(), Some('{'));
                    let num = input.by_ref().take_while(|&x| x != '}');
                    let s_num = String::from_iter(num);
                    u32::from_str_radix(s_num.as_str(), 24)
                        .unwrap()
                        .try_into()
                        .unwrap()
                }
                Some(_) => panic!("Unexpected escape sequence at {:?}", span),
                None => panic!("Unexpected end of String after '\\' at {:?}", span),
            })
        }
    }
    s
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LispType {
    List,
    String,
    Number,
    Symbol,
    Char,
}

impl LispValue {
    pub fn get_type(&self) -> LispType {
        match self {
            LispValue::List(_) => LispType::List,
            LispValue::Number(_) => LispType::Number,
            LispValue::String(_) => LispType::String,
            LispValue::Symbol(_) => LispType::Symbol,
            LispValue::Char(_) => LispType::Char,
            LispValue::Expression(e) => e.get_type(),
        }
    }

    pub fn literal(&self) -> bool {
        match self {
            LispValue::List(_) => false,
            LispValue::Symbol(_) => false,
            LispValue::Expression(e) => e.literal(),
            _ => true,
        }
    }
}

injective_from_pattern!(f64 => LispValue, LispValue::Number);
injective_from_pattern!(char => LispValue, LispValue::Char);
injective_from_pattern!(String => LispValue, LispValue::String);

impl From<bool> for LispValue {
    fn from(b: bool) -> LispValue {
        LispValue::Symbol(String::from(if b { "true" } else { "false" }))
    }
}

impl TryFrom<LispValue> for bool {
    type Error = ConversionError<LispValue, bool>;
    fn try_from(v: LispValue) -> Result<bool, Self::Error> {
        match v {
            LispValue::Symbol(s) => match s.as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(ConversionError::from(LispValue::Symbol(s))),
            },
            _ => Err(ConversionError::from(v)),
        }
    }
}

impl From<&str> for LispValue {
    fn from(s: &str) -> LispValue {
        LispValue::String(String::from(s))
    }
}

impl Default for LispValue {
    fn default() -> LispValue {
        LispValue::Symbol(String::from("undefined"))
    }
}
