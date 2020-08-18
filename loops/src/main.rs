fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Conditional Loops with while
    let mut number = 3;
    
    while number != 0 {
        println!("{}!",number);

        number -= 1;
    }

    // Looping through a collection
    let a = [10,20,30,40,50];

    let mut index = 0;
    while index < 5 {
        println!("The value in the array is {}",a[index]);

        index += 1;
    }

    // Looping through a collection using a for-Loop
    for element in a.iter() {
        println!("The value is: {}",element);
    }

    // Looping through a collection using a for-Loop in Reverse
    for element in (1..4).rev() {
        println!("{}!",element);
    }
    println!("TAKEOFF");
}
