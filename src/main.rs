use anharu_calculator_project::codegen::compile::compile_string;
use std::env::args;
fn main() {
    let input = args().skip(1).next().expect("Failed to get first arg.");

    println!("problem is {}", input);

    match compile_string(&input).expect("Failed to compile.") {
        Some(calc) => unsafe {
            println!("The answer is {}.", calc.call());
        },
        None => println!("failed to get the llvm function."),
    }
}
