use std::io;

fn main() {
    println!("For which number you want the fibunacci number?");
    println!("Enter your number: ");

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Could not read line");

    let number: i32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => 1,
    };

    if number < 1 {
        println!("You entered not a valid fibunacci number. Must be greater than 1 (excluded)")
    } else {
        let result = calculate_fibonacci(number);
        println!("The result of the computation is: {}",result);
    }
}

fn calculate_fibonacci(input: i32) -> i32 {
    match input {
        0 => 1,
        1 => 1,
        _ => calculate_fibonacci(input - 1) + calculate_fibonacci(input - 2),
    }
}