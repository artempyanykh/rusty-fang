mod tast;
mod ty;

fn main() {
    println!("Hello, world!");
    println!("{}", tast::example::const_binding_ex());
    println!("{}", tast::example::fibonacci_ex());
}
