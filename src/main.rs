use anharu_calculator_project::codegen::compile::compile_string;
use std::env::args;
fn main() {
    let input = args().skip(1).next().expect("Failed to get first arg.");

    println!("problem is {}", input);

    println!(
        "The answer is {}",
        compile_string(&input).expect("Failed to compile.")
    );
}
