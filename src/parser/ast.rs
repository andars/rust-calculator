#[deriving(Show)]
pub trait Node {
	fn eval(&self) -> f64;
}

pub struct Num {
	pub num: f64
}

impl Node for Num {
	fn eval(&self) -> f64 {
		self.num
	}
}

pub struct Add {
	pub left: Box<Node>,
	pub right: Box<Node>,
}

impl Node for Add {
	fn eval(&self) -> f64 {
		self.left.eval() + self.right.eval()
	}
}

pub struct Sub {
	pub left: Box<Node>,
	pub right: Box<Node>,
}

impl Node for Sub {
	fn eval(&self) -> f64 {
		self.left.eval() - self.right.eval()
	}
}

pub struct Mul {
	pub left: Box<Node>,
	pub right: Box<Node>,
}

impl Node for Mul {
	fn eval(&self) -> f64 {
		self.left.eval() * self.right.eval()
	}
}

pub struct Div {
	pub left: Box<Node>,
	pub right: Box<Node>,
}

impl Node for Div {
	fn eval(&self) -> f64 {
		self.left.eval() / self.right.eval()
	}
}

pub struct Pow {
	pub base: Box<Node>,
	pub exponent: Box<Node>
}

impl Node for Pow {
	fn eval(&self) -> f64 {
		self.base.eval().powf(self.exponent.eval())
	}
}

pub struct Sin {
	pub arg: Box<Node>
}

impl Node for Sin {
	fn eval(&self) -> f64 {
		self.arg.eval().sin()
	}
}
pub struct Cos {
    pub arg: Box<Node>
}

impl Node for Cos {
    fn eval(&self) -> f64 {
        self.arg.eval().cos()
    }
}

pub struct Sqrt {
	pub arg: Box<Node>
}
impl Node for Sqrt {
	fn eval(&self) -> f64 {
		self.arg.eval().sqrt()
	}
}

pub struct Print {
	pub arg: Box<Node>
}

impl Node for Print {
	fn eval(&self) -> f64 {
		let res = self.arg.eval();
		println!("{}",res);
		res
	}
}
