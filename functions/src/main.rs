//ALL FUNCTIONS ARE EXECUTED AFTER MAIN DID IT
fn main() {
    println!("Hello, world!");
    let x = five();

    println!("The value of x is: {x}");

    function();
    func_1();
}

fn function() {
    println!("Hello from other function");

    sum_two_numbers(34, 100);
}

//parameters
fn sum_two_numbers(value_1: i32, value_2: i32) {
    let sum: i32 = value_1 + value_2;
    println!("{}", sum);
}

//statements and expressions
//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resulting value.
//statement >> declare the function
//expression >> call the function
fn func_1() {
    //statement
    let y = 6;
    //expression
    let z = {
        let x = 3;
        // Expressions do not include ending semicolons
        x + 1
    };

    println!("value statement: {y} --- value expression: {z}");
}

//return values
//use fn name() -> type {}
fn five() -> i32 {
    5
}

//can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly

//CONTROL FLOW
fn flow() {
    let number = 6;
    //adding to a let stament the expression if
    let number_2 = if number > 4 { 5 } else { 8 };

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
