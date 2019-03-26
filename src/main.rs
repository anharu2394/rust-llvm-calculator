use anharu_calculator_project::codegen::compile::compile_string;
use std::env::args;
fn main() {
    let input = args().skip(1).next().expect("Failed to get first arg.");

    println!("problem is {}", input);

    let calc = compile_string(&input).expect("Failed to compile.");

    unsafe {
        println!("The answer is {}", calc.call());
    }
}
