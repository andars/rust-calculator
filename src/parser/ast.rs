extern crate std;
use std::collections::HashMap;
use std::f64;

pub trait Node {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64>;
}

pub struct Num {
    pub num: f64
}

impl Node for Num {
    fn eval(&self, _env: &mut HashMap<String, f64>) -> Option<f64> {
        Some(self.num)
    }
}

pub struct Add {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl Node for Add {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.left.eval(env) {
            Some(l) => {
                match self.right.eval(env) {
                    Some(r) => Some(l + r),
                    None => None
                }
            }
            None => None
        }
    }
}

pub struct Sub {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl Node for Sub {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.left.eval(env) {
            Some(l) => {
                match self.right.eval(env) {
                    Some(r) => Some(l - r),
                    None => None
                }
            }
            None => None
        }
    }
}

pub struct Mul {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl Node for Mul {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.left.eval(env) {
            Some(l) => {
                match self.right.eval(env) {
                    Some(r) => Some(l*r),
                    None => None
                }
            }
            None => None
        }
    }
}

pub struct Div {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl Node for Div {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.left.eval(env) {
            Some(l) => {
                match self.right.eval(env) {
                    Some(r) => Some(l/r),
                    None => None
                }
            }
            None => None
        }
    }
}

pub struct Pow {
    pub base: Box<Node>,
    pub exponent: Box<Node>
}

impl Node for Pow {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.base.eval(env) {
            Some(b) => {
                match self.exponent.eval(env) {
                    Some(e) => Some(b.powf(e)),
                    None => None
                }
            }
            None => None
        }
    }
}

pub struct Sin {
    pub arg: Box<Node>
}

impl Node for Sin {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.arg.eval(env) {
            Some(x) => Some(x.sin()),
            None => None
        }
    }
}
pub struct Cos {
    pub arg: Box<Node>
}

impl Node for Cos {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.arg.eval(env) {
            Some(x) => Some(x.cos()),
            None => None
        }
    }
}

pub struct Sqrt {
    pub arg: Box<Node>
}
impl Node for Sqrt {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match self.arg.eval(env) {
            Some(x) => Some(x.sqrt()),
            None => None
        }
    }
}

pub struct Print {
    pub arg: Box<Node>
}

impl Node for Print {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        let res = self.arg.eval(env);
        match res {
            Some(x) => println!("{}", x),
            None => {}
        }
        res
    }
}

pub struct Var {
    pub name: String
}

impl Node for Var {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        match env.get(&(self.name)[..]) {
            Some(r) => Some(*r),
            None => None
        }
    }
}

pub struct Assignment {
    pub name: String,
    pub value: Box<Node>
}

impl Node for Assignment {
    fn eval(&self, env: &mut HashMap<String, f64>) -> Option<f64> {
        let val = self.value.eval(env);
        match val {
            Some(x) => { env.insert(self.name.clone(), x); }
            None => {}
        }
        val
    }
}
