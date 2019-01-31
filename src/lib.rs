
use std::rc::Rc;
use std::fmt;

pub trait Function: fmt::Display {

    fn evaluate(&self, x: &f64) -> f64;
    fn diff(&self) -> Rc<dyn Function>;
}

pub struct SumFunction {

    right: Rc<dyn Function>,
    left: Rc<dyn Function>,
}

impl SumFunction {

    pub fn new(left: Rc<dyn Function>, right: Rc<dyn Function>) -> Rc<dyn Function> {

        Rc::new(SumFunction { left, right })
    }
}

impl Function for SumFunction {

    fn evaluate(&self, x: &f64) -> f64 {

        self.left.evaluate(x) + self.right.evaluate(x)
    }

    fn diff(&self) -> Rc<dyn Function> {
        
        SumFunction::new(self.left.diff(), self.right.diff())
    }
}

impl fmt::Display for SumFunction {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "({} + {})", self.left.to_string(), self.right.to_string())
    }
}

pub struct ProductFunction {

    left: Rc<dyn Function>,
    right: Rc<dyn Function>,
}

impl ProductFunction {

    pub fn new(left: Rc<dyn Function>, right: Rc<dyn Function>) -> Rc<dyn Function> {

        Rc::new(ProductFunction { left, right })
    }
}

impl Function for ProductFunction {

    fn evaluate(&self, x: &f64) -> f64 {

        self.left.evaluate(x) * self.right.evaluate(x)
    }

    fn diff(&self) -> Rc<dyn Function> {

        let l_diff = self.left.diff();
        let r_diff = self.right.diff();

        let l_clone = Rc::clone(&self.left);
        let r_clone = Rc::clone(&self.right);

        let l_term = ProductFunction::new(l_diff, r_clone);
        let r_term = ProductFunction::new(r_diff, l_clone);

        SumFunction::new(l_term, r_term)
    }
}

impl fmt::Display for ProductFunction {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "({} * {})", self.left.to_string(), self.right.to_string())
    }
}

pub struct ComposedFunction {

    target: Rc<dyn Function>,
    source: Rc<dyn Function>,
}

impl ComposedFunction {

    pub fn new(source: Rc<dyn Function>, target: Rc<dyn Function>) -> Rc<dyn Function> {

        Rc::new(ComposedFunction { target, source })
    }
}

impl Function for ComposedFunction {

    fn evaluate(&self, x: &f64) -> f64 {

        self.source.evaluate(&self.target.evaluate(x))
    }

    fn diff(&self) -> Rc<dyn Function> {

        let s_diff = self.source.diff();
        let t_diff = self.target.diff();

        let t_clone = Rc::clone(&self.target);

        ProductFunction::new(ComposedFunction::new(s_diff, t_clone), t_diff)
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

    Constant(f64),
    Id,
    Sine,
    Cosine,
    Exp,
}

impl UnaryFunction {

    pub fn new(kind: UnaryFunction) -> Rc<dyn Function> {

        Rc::new(kind)
    }
}

impl Function for UnaryFunction {

    fn evaluate(&self, x: &f64) -> f64 {

        match self {

            UnaryFunction::Constant(c) => *c,
            UnaryFunction::Id => *x,
            UnaryFunction::Sine => x.sin(),
            UnaryFunction::Cosine => x.cos(),
            UnaryFunction::Exp => x.exp(),
        }
    }

    fn diff(&self) -> Rc<dyn Function> {

        match self {
            
            UnaryFunction::Constant(_) => UnaryFunction::new(UnaryFunction::Constant(0.0)),
            UnaryFunction::Id => UnaryFunction::new(UnaryFunction::Constant(1.0)),
            UnaryFunction::Sine => UnaryFunction::new(UnaryFunction::Cosine),
            UnaryFunction::Cosine => ProductFunction::new(UnaryFunction::new(UnaryFunction::Constant(-1.0)), UnaryFunction::new(UnaryFunction::Sine)),
            UnaryFunction::Exp => UnaryFunction::new(UnaryFunction::Exp),
        }
    }
}

impl fmt::Display for UnaryFunction {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let plain = match self {

            UnaryFunction::Constant(c) => c.to_string(),
            UnaryFunction::Id => String::from("x"),
            UnaryFunction::Sine => String::from("sin(x)"),
            UnaryFunction::Cosine => String::from("cos(x)"),
            UnaryFunction::Exp => String::from("exp(x)"),
        };

        write!(f, "({})", plain)
    }
}

