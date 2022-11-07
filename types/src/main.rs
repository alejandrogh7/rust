fn main() {
    //theres just one way to have mutable variables that is adding mut
    //to the variable, in other cases we are goind to have an error
    //when we change the value of the variable
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //---CONSTANTS---
    //are not allowed to change
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{}", THREE_HOURS_IN_SECONDS);
    //Constants are valid for the entire time a program runs, within the scope they were declared in

    //---SHADOWING---
    //we can declare a new variable with the same name as a previous variable
    //means that we 'shadowed' the F variable with the S variable
    let x = 5;
    //6
    let x = x + 1;

    {
        //we’re effectively creating a new variable when we use the let keyword again
        //6 * 2 = 12
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    //the scope 'X' variable is 6
    println!("The value of x is: {x}");

    //---DATA TYPES---
    //1 --- >> Scalar types:integers, floating-point numbers, Booleans, and characters

    //------ >> INTEGER
    //format (Signed and unsigned)(LENGTH -->> 8, 16, 32, 64, 128)
    //Each signed variant can store numbers from where 'n' is the length
    //-(2*(n - 1)) to (2**(n - 1)) - 1 >>> FOR SIGNED
    //0 to (2**(n - 1)) - 1 >>> FOR UNSIGNED
    let integer_1: i32 = 1_000_000;
    let integer_2: u8 = 254;

    println!("INT = {integer_1} and {integer_2}");

    //------ >> FLOAT
    //we have (f32 and f64)
    //which are 32 bits and 64 bits in size
    //f64 is the default type
    let float_1 = 2.0;

    let float_2: f32 = 3.0;

    println!("FLOAT = {float_1} and {float_2}");

    //------ >> BOOLEAN
    let t = true;

    let f: bool = false;

    //------ >> CHARACTER
    //char type is the language’s most primitive alphabetic type.
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //------ >> OPERATIONS
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    //2 --- >> Compound Type: tuples and arrays.

    //------ >> TUPLE
    //Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    //can any type
    //The tuple without any values has a special name, unit
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //use pattern matching to destructure a tuple value
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    //we can also access to an element inside a tuple using '.index'
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    //------ >> ARRAY
    //every element of an array must have the same type
    // have a fixed length.
    //arrays are more useful when you know the number of elements will not need to change.
    //[define-type; number-of-elements]
    let a:[i32; 5] = [1, 2, 3, 4, 5];
    // can also initialize an array to contain the same value for each element
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let january = months[0];
}
