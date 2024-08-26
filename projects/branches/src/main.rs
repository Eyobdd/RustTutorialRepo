fn main() {
    let number = 6;

    // Basic Condition (no concept of truthy, must be a boolean expression)
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // Equivalent of a truthly check for a Integer and Float values
    if number != 0 {
        println!("number was something other than zero")
    }

    // Multiple if else conditions
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // Even though 6 is divisible by both 3 and 2 the first true condition takes precedence .
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Can use `if` in a let statement
    let condition = true;
    // Both branches of the conditional must have the same return type
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    /* Loops */

    // `loop` Loops
    let mut counter = 0;
    let result = loop {
        println!("new count: {counter}");
        counter += 1;
        if counter == 10 {
            println!("break! {counter}");
            // break exits the current loop whereas return exits the current function
            break counter * 2;
        }
    };
    println!("Result value is: {result}");

    // Loop Labels
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // `while` Loops
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!");

    // `for` Loops
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("the value is: {element}");
    }

    // Liftoff with a for loop
    for number in (1..=3).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
