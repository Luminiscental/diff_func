
use diff_func::*;

fn diff_and_print(name: &str, f: Function) {

    println!("{name}(x) = {}, {name}'(x) = {}", f, f.diff(), name = name);
}

fn main() {

    let sinc = UnaryFunction::Sin.new().div(UnaryFunction::Id.new());
    let log_div_x = UnaryFunction::Log.new().div(UnaryFunction::Id.new());
    let llc = UnaryFunction::Log.new().of(UnaryFunction::Log.new().of(UnaryFunction::Cos.new()));

    diff_and_print("f", sinc);
    diff_and_print("g", log_div_x);
    diff_and_print("h", llc);
}
