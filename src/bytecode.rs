use core::fmt::Debug;
use std::collections::HashMap;
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::hash::Hash;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Bytecode<S, V> {
    PushConst(V),
    Push(S),
    Pop(S),
    StoreConst(V, S),
    Load(S),
    Call(S),
    Add,
    Sub,
    Mul,
    Div,
    Lte,
    Equ,
    Not,
}

#[derive(Debug)]
pub struct Compiled<S: Eq + Hash, V>(HashMap<S, Vec<Bytecode<S, V>>>);

pub trait AstValue {
    type OutputValue;
    type SymbolValue: Eq + Hash;
    fn compile(&self) -> Compiled<Self::SymbolValue, Self::OutputValue>;
}

trait CatchRuntimeError<T> {
    fn not_enough_arguments(self, ctxt: &str) -> T;
    fn wrong_argument_type(self, ctxt: &str) -> T;
}

impl<T> CatchRuntimeError<T> for Option<T> {
    fn not_enough_arguments(self, ctxt: &str) -> T {
        self.unwrap_or_else(|| panic!("{}: not enough arguments", ctxt))
    }
    fn wrong_argument_type(self, ctxt: &str) -> T {
        self.unwrap_or_else(|| panic!("{}: wrong argument type", ctxt))
    }
}

impl<T, E: Debug> CatchRuntimeError<T> for Result<T, E> {
    fn not_enough_arguments(self, ctxt: &str) -> T {
        match self {
            Ok(x) => x,
            Err(x) => panic!("{}: not enough arguments - {:?}", ctxt, x),
        }
    }
    fn wrong_argument_type(self, ctxt: &str) -> T {
        match self {
            Ok(x) => x,
            Err(x) => panic!("{}: wrong argument types - {:?}", ctxt, x),
        }
    }
}

pub trait Heap<S, V> {
    fn insert(&mut self, symbol: S, value: V);
    fn get(&self, symbol: &S) -> Option<V>;
}

impl<S: Eq + Hash + Clone, V: Clone> Heap<S, V> for HashMap<S, V> {
    fn insert(&mut self, symbol: S, value: V) {
        self.insert(symbol, value);
    }
    fn get(&self, symbol: &S) -> Option<V> {
        self.get(symbol).cloned()
    }
}

pub trait Entry {
    fn entry() -> Self;
}

#[allow(dead_code)]
impl<S: Eq + Hash + Clone + Entry + Debug, V> Compiled<S, V>
where
    V: From<f64> + From<bool> + From<S> + Clone + Default + PartialEq,
    f64: TryFrom<V>,
    bool: TryFrom<V>,
    S: TryFrom<V>,
    <f64 as TryFrom<V>>::Error: Debug,
    <bool as TryFrom<V>>::Error: Debug,
    <S as TryFrom<V>>::Error: Debug,
{
    pub fn run(&self, entry: &S, stack: &mut Vec<V>, heap: &mut dyn Heap<S, V>) {
        for op in self.0.get(entry).unwrap_or(&Vec::new()) {
            match op {
                Bytecode::PushConst(val) => stack.push(val.clone()),
                Bytecode::Push(sym) => stack.push(heap.get(sym).unwrap_or_default()),
                Bytecode::Pop(sym) => {
                    heap.insert(sym.clone(), stack.pop().unwrap_or_default());
                }
                Bytecode::StoreConst(val, sym) => {
                    heap.insert(sym.clone(), val.clone());
                }
                Bytecode::Load(sym) => stack.push(heap.get(sym).unwrap_or_default()),
                Bytecode::Call(sym) => {
                    self.run(sym, stack, heap);
                }
                Bytecode::Add => {
                    let a: V = stack.pop().not_enough_arguments("Add");
                    let b: V = stack.pop().not_enough_arguments("Add");
                    let a = TryInto::<f64>::try_into(a).wrong_argument_type("Add");
                    let b = TryInto::<f64>::try_into(b).wrong_argument_type("Add");
                    stack.push(V::from(a + b));
                }
                Bytecode::Sub => {
                    let a: V = stack.pop().not_enough_arguments("Sub");
                    let b: V = stack.pop().not_enough_arguments("Sub");
                    let a = TryInto::<f64>::try_into(a).wrong_argument_type("Sub");
                    let b = TryInto::<f64>::try_into(b).wrong_argument_type("Sub");
                    stack.push(V::from(a - b));
                }
                Bytecode::Div => {
                    let a: V = stack.pop().not_enough_arguments("Div");
                    let b: V = stack.pop().not_enough_arguments("Div");
                    let a = TryInto::<f64>::try_into(a).wrong_argument_type("Div");
                    let b = TryInto::<f64>::try_into(b).wrong_argument_type("Div");
                    stack.push(V::from(a / b));
                }
                Bytecode::Mul => {
                    let a: V = stack.pop().not_enough_arguments("Mul");
                    let b: V = stack.pop().not_enough_arguments("Mul");
                    let a = TryInto::<f64>::try_into(a).wrong_argument_type("Mul");
                    let b = TryInto::<f64>::try_into(b).wrong_argument_type("Mul");
                    stack.push(V::from(a * b));
                }
                Bytecode::Equ => {
                    let a: V = stack.pop().not_enough_arguments("Equ");
                    let b: V = stack.pop().not_enough_arguments("Equ");
                    stack.push(V::from(a == b));
                }
                Bytecode::Not => {
                    let a: V = stack.pop().not_enough_arguments("Not");
                    let a = TryInto::<bool>::try_into(a).wrong_argument_type("Not");
                    stack.push(V::from(!a));
                }
                Bytecode::Lte => {
                    let a: V = stack.pop().not_enough_arguments("Lte");
                    let b: V = stack.pop().not_enough_arguments("Lte");
                    let a = TryInto::<f64>::try_into(a).wrong_argument_type("Lte");
                    let b = TryInto::<f64>::try_into(b).wrong_argument_type("Lte");
                    stack.push(V::from(a <= b));
                }
            }
        }
    }

    pub fn add_entry(&mut self, k: S, v: Vec<Bytecode<S, V>>) {
        match self.0.get_mut(&k) {
            Some(e) => for c in v { e.push(c); }
            None => { self.0.insert(k, v); }
        }
    }

    pub fn append(&mut self, c: Bytecode<S, V>) {
        self.0.get_mut(&S::entry()).unwrap().push(c);
    }

    pub fn append_compiled(&mut self, c: Compiled<S, V>) {
        for sym in c.0 {
            if sym.0 == S::entry() {
                self.add_entry(S::entry(), sym.1);
                continue;
            }

            if self.0.contains_key(&sym.0) {
                panic!("Redifinition of function symbol: {:?}", sym.0);
            } else {
                self.add_entry(sym.0, sym.1);
            }
        }
    }

    pub fn entry(&self) -> &Vec<Bytecode<S, V>> {
        match self.0.get(&S::entry()) {
            Some(expr) => expr,
            None => panic!("No entry!"),
        }
    }

    pub fn new() -> Compiled<S, V> {
        let mut m = HashMap::new();
        m.insert(S::entry(), Vec::new());
        Compiled(m)
    }
}
