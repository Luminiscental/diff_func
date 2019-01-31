
use diff_func::*;

fn main() {

    let sqr_func = ProductFunction::new(UnaryFunction::new(UnaryFunction::Id), UnaryFunction::new(UnaryFunction::Id));
    let sqr_diff = sqr_func.diff();

    println!("x^2 = {}, d/dx(x^2) = {}", sqr_func, sqr_diff);
}
