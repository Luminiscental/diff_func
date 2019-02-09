
use diff_func::*;

fn diff_and_print(name: &str, f: Function) {

    println!("{name}(x) = {}, {name}'(x) = {}", f.to_string().replace("$", "x"), f.diff().to_string().replace("$", "x"), name = name);
}

fn main() {

    let simple = UnaryFunction::Const(2.0).new().mul(UnaryFunction::Id.new()).expand();
    let sinc = UnaryFunction::Sin.new().div(UnaryFunction::Id.new()).expand();
    let log_div_x = UnaryFunction::Log.new().div(UnaryFunction::Id.new()).expand();
    let llc = UnaryFunction::Log.new().of(UnaryFunction::Log.new().of(UnaryFunction::Cos.new())).expand();

    diff_and_print("a", simple);
    diff_and_print("f", sinc);
    diff_and_print("g", log_div_x);
    diff_and_print("h", llc);
}
