mod interp;
mod tast;
mod ty;

use std::rc::Rc;

fn main() {
    let env = interp::Env::new();

    let ex = Rc::new(tast::example::const_binding_ex());
    println!("{}", ex);
    println!("=== Eval");
    println!("{}", interp::eval_ex(ex, &env));

    let fib_name = tast::NameDef("fib".to_string());
    let fib_binding = tast::example::fibonacci();
    let env = env.bind(&fib_name, Rc::new(fib_binding.ex));
    let ex = Rc::new(tast::example::fibonacci_ex_n(1));
    println!("{}", ex);
    println!("=== Eval");
    println!("{}", interp::eval_ex(ex, &env));
}
