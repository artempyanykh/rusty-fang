mod builtin;
mod interp;
mod tast;
mod ty;

fn main() {
    let env = interp::Env::new();

    // let ex = Rc::new(tast::example::const_binding_ex());
    // println!("{}", ex);
    // println!("=== Eval");
    // println!("{}", interp::eval_ex(ex, &env));

    // let inc_name = tast::NameDef("inc".to_string());
    // let inc_binding = tast::example::inc_binding();
    // println!("{}", inc_binding.ex);
    // let env = env.bind(&inc_name, Rc::new(inc_binding.ex));
    // let ex = Rc::new(tast::example::incinc_ap_n(15));
    // println!("{}", ex);
    // println!("=== Eval");
    // println!("{}", interp::eval_ex(ex, &env));

    let fib_name = tast::NameDef("fib".to_string());
    let fib_binding = tast::example::fibonacci();
    println!("{}", fib_binding.ex);
    let env = env.bind(&fib_name, fib_binding.ex);
    let ex = tast::example::fibonacci_ap_n(30);
    println!("{}", ex);
    println!("=== Eval");
    println!("{}", interp::eval_ex(ex, &env));
}
