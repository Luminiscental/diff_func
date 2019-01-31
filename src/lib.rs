
use std::rc::Rc;
use std::fmt;

pub type Function = Rc<dyn FunctionTrait>;

pub trait FunctionTrait: fmt::Display {

    fn evaluate(&self, x: &f64) -> f64;
    fn diff(&self) -> Function;
}

pub trait FunctionOf {

    fn of(self, other: Self) -> Self;
}

pub trait FunctionAdd {

    fn add(self, other: Self) -> Self;
}

pub trait FunctionMul {

    fn mul(self, other: Self) -> Self;
}

pub trait FunctionDiv {

    fn div(self, other: Self) -> Self;
}

impl FunctionOf for Function {

    fn of(self, other: Function) -> Function {

        ComposedFunction::new(self, other)
    }
}

impl FunctionAdd for Function {

    fn add(self, other: Function) -> Function {

        SumFunction::new(self, other)
    }
}

impl FunctionMul for Function {

    fn mul(self, other: Function) -> Function {

        ProductFunction::new(self, other)
    }
}

impl FunctionDiv for Function {

    fn div(self, other: Function) -> Function {

        let log_of = UnaryFunction::Log.new().of(self);
        let neg_log = UnaryFunction::Const(-1.0).new().mul(log_of);
        let reciprocal = UnaryFunction::Exp.new().of(neg_log);

        other.mul(reciprocal)
    }
}

pub struct SumFunction {

    right: Function,
    left: Function,
}

impl SumFunction {

    pub fn new(left: Function, right: Function) -> Function {

        Rc::new(SumFunction { left, right })
    }
}

impl FunctionTrait for SumFunction {

    fn evaluate(&self, x: &f64) -> f64 {

        self.left.evaluate(x) + self.right.evaluate(x)
    }

    fn diff(&self) -> Function {
        
        self.left.diff().add(self.right.diff())
    }
}

impl fmt::Display for SumFunction {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "({} + {})", self.left.to_string(), self.right.to_string())
    }
}

pub struct ProductFunction {

    left: Function,
    right: Function,
}

impl ProductFunction {

    pub fn new(left: Function, right: Function) -> Function {

        Rc::new(ProductFunction { left, right })
    }
}

impl FunctionTrait for ProductFunction {

    fn evaluate(&self, x: &f64) -> f64 {

        self.left.evaluate(x) * self.right.evaluate(x)
    }

    fn diff(&self) -> Function {

        let l_diff = self.left.diff();
        let r_diff = self.right.diff();

        let l_clone = Rc::clone(&self.left);
        let r_clone = Rc::clone(&self.right);

        let l_term = l_diff.mul(r_clone);
        let r_term = r_diff.mul(l_clone);

        l_term.add(r_term)
    }
}

impl fmt::Display for ProductFunction {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "({} * {})", self.left.to_string(), self.right.to_string())
    }
}

pub struct ComposedFunction {

    target: Function,
    source: Function,
}

impl ComposedFunction {

    pub fn new(source: Function, target: Function) -> Function {

        Rc::new(ComposedFunction { target, source })
    }
}

impl FunctionTrait for ComposedFunction {

    fn evaluate(&self, x: &f64) -> f64 {

        self.source.evaluate(&self.target.evaluate(x))
    }

    fn diff(&self) -> Function {

        let s_diff = self.source.diff();
        let t_diff = self.target.diff();

        let t_clone = Rc::clone(&self.target);

        s_diff.of(t_clone).mul(t_diff)
    }
}

impl fmt::Display for ComposedFunction {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let source_of_x = self.source.to_string();
        let target_of_x = self.target.to_string();

        write!(f, "{}", source_of_x.replace("x", &target_of_x))
    }
}

pub enum UnaryFunction {

    Const(f64),
    Id,
    Sin,
    Cos,
    Exp,
    Log,
}

impl UnaryFunction {

    pub fn new(self) -> Function {

        Rc::new(self)
    }
}

impl FunctionTrait for UnaryFunction {

    fn evaluate(&self, x: &f64) -> f64 {

        match self {

            UnaryFunction::Const(c) => *c,
            UnaryFunction::Id => *x,
            UnaryFunction::Sin => x.sin(),
            UnaryFunction::Cos => x.cos(),
            UnaryFunction::Exp => x.exp(),
            UnaryFunction::Log => x.ln(),
        }
    }

    fn diff(&self) -> Function {

        match self {
            
            UnaryFunction::Const(_) => UnaryFunction::Const(0.0).new(),
            UnaryFunction::Id => UnaryFunction::Const(1.0).new(),
            UnaryFunction::Sin => UnaryFunction::Cos.new(),
            UnaryFunction::Cos => UnaryFunction::Const(-1.0).new().mul(UnaryFunction::Sin.new()),
            UnaryFunction::Exp => UnaryFunction::Exp.new(),
            UnaryFunction::Log => UnaryFunction::Const(1.0).new().div(UnaryFunction::Id.new()),
        }
    }
}

impl fmt::Display for UnaryFunction {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let plain = match self {

            UnaryFunction::Const(c) => c.to_string(),
            UnaryFunction::Id => String::from("x"),
            UnaryFunction::Sin => String::from("sin(x)"),
            UnaryFunction::Cos => String::from("cos(x)"),
            UnaryFunction::Exp => String::from("exp(x)"),
            UnaryFunction::Log => String::from("ln(x)"),
        };

        write!(f, "({})", plain)
    }
}

