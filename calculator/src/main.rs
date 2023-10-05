// Steps
//     Create an enum called Operation with variants Add, Subtract, Multiply, and Divide. Each variant should hold two f64 values.
//     Create a function called calculate that takes an Operation enum as an argument and returns an f64 result.
//     Implement the calculate function using pattern matching to perform the appropriate arithmetic operation based on the variant of the Operation enum.
//     In the main function, prompt the user to input the first number, the operation to be performed, and the second number.
//     Parse the user input into appropriate variables.
//     Create an Operation enum instance with the parsed input values.
//     Call the calculate function with the created Operation enum instance.
//     Print the result to the console.
//     Compile and run the program to ensure it works as expected.

use std::io::*;

enum Operation {
    Add (f64, f64),
    Subtract(f64, f64), 
    Multiply(f64, f64),
    Divide(f64, f64)
}


fn calculate(args: Operation) -> f64 {
    match args {
        Operation::Add(a, b) => a+b,
        Operation::Subtract(a, b) => a-b,
        Operation::Multiply(a, b) => a*b,
        Operation::Divide(a, b) => a/b,
    }
}

fn prompt(stdin: &Stdin, prompt: &str)-> String {
    let mut user_input = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    let _ = stdin.read_line(&mut user_input);

    return String::from(user_input);
    
}


fn main() {
    let stdin = stdin();
        
    let a: f64 = prompt(&stdin, "gimme number: ").trim().parse().unwrap();
    let operator: char = prompt(&stdin, "which operator (a=add, s=sub, m=mul, d=div) ? ").trim().parse().unwrap();
    let b: f64 = prompt(&stdin, "gimme number: ").trim().parse().unwrap();


    let op: Operation = match operator {
        'a' => Operation::Add (a, b),
        's' => Operation::Subtract (a, b),
        'm' => Operation::Multiply (a, b),
        'd' => Operation::Divide (a, b),
        _ => Operation::Add (a, b),             // error cases for another day
    };   

    let result = calculate(op);

    println!("resuuuult {}", result);
}
