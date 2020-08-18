
fn main() {
    // let mut x = 5;
    // println!("The value of x is: {}",x);
    // x = 6;
    // println!("The value of x is: {}",x);

    // let x = 5;

    // let x = x + 1;
    
    // let x = x * 2;

    // println!("The value of x is: {}",x);

    // let spaces = "    ";

    // let spaces = spaces.len();

    // println!("The count of spaces is: {}", spaces);

    // Numerical operations
    // addition
    let sum = 5 + 10;
    println!("The sum is: {}",sum);

    // substraction
    let difference = 95.5 - 53.4;
    println!("The difference is: {}",difference);

    // multiplication
    let product = 4 * 30;
    println!("The result of multiplication is: {}",product);

    // remainder = Modulo
    let remainder = 43 % 5;
    println!("The rest is: {}",remainder);

    // Character variables
    let _c = 'z';
    let _z = 'Z';
    let heart_eyed_cat = '😻';
    println!("{}",heart_eyed_cat);

    // Tuples
    let tup: (i32, f64, _)= (100, 10.5, "a");

    // Destructuring of a tuple
    let (_a, _b, d) = tup;

    println!("The value of d is: {}",d);

    // Or direct access on an element of the tuple
    let _v = tup.2; // = "a"

    // Array
    let _array: [i32; 5] = [1,2,3,4,5];

    // Producing a array with initial values
    let _array_of_threes = [3; 5];

    another_function(13,14);

    // Definition of expressions and statements
    // Every line of code with a ';' at the end is a statement and return no value
    // Every line of code without a ';' is a expression and returns something
    // --> Important for return Type of functions
    let y = {
        let i = 3;
        i + 1
    };
    println!("The value of y is: {}",y);

    let five = return_five();
    println!("The value of five is: {}",five);

    let addition = add(44, 22);
    println!("The result of the addition is: {}",addition);
}

// Writing another function
// fn another_function() {
//     println!("Another function!");
// }
// Another function with a parameter
fn another_function(x: i32,y: i32) {
    println!("The value of x is: {}",x);
    println!("The value of y is: {}",y);
}

// Function with a return element
fn return_five() -> i32 {
    5
}

// Function with parameters and a computed return value
fn add(x: i32, y: i32) -> i32 {
    x + y
}