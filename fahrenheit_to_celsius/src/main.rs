use std::io; 

fn main() {
    loop {
        println!("What do you want to convert? (1,2,3)");
        println!("1) Celsius to Fahrenheit ");
        println!("2) Fahrenheit to Celsius");
        println!("3) Exit");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Something bad happend...");

        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 3 {
            println!("You exited the programm!");
            break;
        } else if choice == 2 {
            println!("What's your Fahrenheit value: ");

            let mut fahrenheit = String::new();

            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Could not read text");

            let fahrenheit: i64 = match fahrenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let celsius = fahrenheit_to_celsius(fahrenheit);

            println!("The converted fahrenheit value {} is in celsius {}", fahrenheit as i8,celsius as i8);

        } else if choice == 1 {
            println!("What's your Celsius value: ");

            let mut celsius = String::new();

            io::stdin()
                .read_line(&mut celsius)
                .expect("Could not read text");

            let celsius: i64 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let fahrenheit = celsius_to_fahrenheit(celsius);

            println!("The converted celsius value {} is in fahrenheit {}",celsius as i8,fahrenheit as i8);
        }

    }
}

fn fahrenheit_to_celsius(fahrenheit: i64) -> i64 {
    let result: i64 = (fahrenheit - 32) * (5/9);
    result
}


fn celsius_to_fahrenheit(celsius: i64) -> i64 {
    (celsius * (9/5)) + 32 
}