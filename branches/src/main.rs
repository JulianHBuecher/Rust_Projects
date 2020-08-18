fn main() {
    let number = 3;

    if number < 5 {
        println!("The condition was true!");
    } else {
        println!("The condition was false!")
    }

    // Another possibility
    if number != 0 {
        println!("The number was something other than 0");
    }

    let large_number: i64 = 123_456;
    let last_digit = large_number % 10;
    println!("The last digit of large_number is: {}",last_digit);

    let number = 6;

    // If, Else If and Else
    if number % 4 == 0 {
        println!("The number is divisible by 4");
    } else if number % 3 == 0 {
        println!("The number is divisible by 3");
    } else if number % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4, 3 or 2!");
    }

    println!("The rest of number % 2 is: {}",number % 2);

    // If in let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {}",number);
}
