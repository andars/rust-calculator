extern crate std;
use std::num::Float;
use std::collections::HashMap;

pub trait Node {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64;
}

pub struct Num {
    pub num: f64
}

impl Node for Num {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        self.num
    }
}

pub struct Add {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl Node for Add {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        self.left.eval(env) + self.right.eval(env)
    }
}

pub struct Sub {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl Node for Sub {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        self.left.eval(env) - self.right.eval(env)
    }
}

pub struct Mul {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl Node for Mul {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        self.left.eval(env) * self.right.eval(env)
    }
}

pub struct Div {
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl Node for Div {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        self.left.eval(env) / self.right.eval(env)
    }
}

pub struct Pow {
    pub base: Box<Node>,
    pub exponent: Box<Node>
}

impl Node for Pow {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        std::num::Float::powf(self.base.eval(env), self.exponent.eval(env))
    }
}

pub struct Sin {
    pub arg: Box<Node>
}

impl Node for Sin {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        self.arg.eval(env).sin()
    }
}
pub struct Cos {
    pub arg: Box<Node>
}

impl Node for Cos {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        self.arg.eval(env).cos()
    }
}

pub struct Sqrt {
    pub arg: Box<Node>
}
impl Node for Sqrt {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        self.arg.eval(env).sqrt()
    }
}

pub struct Print {
    pub arg: Box<Node>
}

impl Node for Print {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        let res = self.arg.eval(env);
        println!("{}",res);
        res
    }
}

pub struct Var {
    pub name: String
}

impl Node for Var {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        return *env.get(&(self.name)[..]).unwrap();        
    }
}

pub struct Assignment {
    pub name: String,
    pub value: Box<Node>
}

impl Node for Assignment {
    fn eval(&self, env: &mut HashMap<String, f64>) -> f64 {
        let val = self.value.eval(env);
        env.insert(self.name.clone(), val);
        val
    }
}
