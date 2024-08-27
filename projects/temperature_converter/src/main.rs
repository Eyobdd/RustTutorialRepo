use std::io;

fn main() {
    println!("Welcome to Temperature Converter.");
    // Controls whether while loop continues based on input
    let mut continue_converting = true;
    while continue_converting {
        println!("\nEnter the temperature you world like to with its unit.\n");
        println!("i.e. 10 ˚F would be 10F and 1.2 ˚C would be 1.2C\n");
        // Get Input
        let input = get_text_input();
        // Parse input for valid float value and 'C' or 'F' last character
        let (temp, unit) = (get_temp(&input), get_temp_unit(&input));
        // If both `temp` and `unit` successfully parse input string calculate
        // converted temperature. Else, print error message.
        match (temp, unit) {
            (Some(temp), Some(unit)) => {
                continue_converting = false;
                println!("\nParsed Input: {temp}˚{unit}");
                println!("Converted Value: {}", convert_temp(&temp, &unit));
            }
            _ => println!("[Unable to parse text.]"),
        }
        // If a value was converted prompt user to see if they would like to
        // continue.
        if !continue_converting {
            // Prompt user
            println!("\nDo you want to continue converting temperatures?\n\n[TYPE Y TO CONTINUE]\n\n[PRESS ENTER TO QUIT]\n");
            // Check if user input has typed the letter "Y"
            if get_text_input().to_uppercase() == "Y" {
                continue_converting = true;
            }
        }
    }
}
/* `get_text_input` retrieves the last line inputted by the user. */
fn get_text_input() -> String {
    let mut input = String::new();

    let _ = io::stdin().read_line(&mut input);

    return input.trim().to_string();
}
/*
 `get_temp_unit` parses for the last character of a string. If the chararacter is
 'F' or 'C', return a `Some<String>` else return `None`.
*/
fn get_temp_unit(str: &str) -> Option<String> {
    str.to_uppercase()
        .trim()
        .chars()
        .last()
        .and_then(|last_char| match last_char {
            'F' | 'C' => Some(last_char.to_string()),
            _ => None,
        })
}
/*
 `get_temp` removes non-numeric characters from a `String` and parses the
 remaining `String` into a `f64` float. If parse fails, return `None`; else,
 return `Some<f64>`;
*/
fn get_temp(str: &str) -> Option<f64> {
    let value: String = str
        .chars()
        .filter(|c| c.is_digit(10) || *c == '.' || *c == '-')
        .collect();
    return match value.parse() {
        Ok(val) => Some(val),
        Err(_) => None,
    };
}
/*
 `convert_temp` takes a `f64` float value and depending on the value of the
 temperature unit is 'F' or 'C' converts the float using either the fahrenheit
 to celsius or celsius to fahrenheit formulas. The new temperature value is
 rounded to two decimal places and concatenated with '˚' and the new
 temperature unit before being returned as a `String`.
*/
fn convert_temp(temp: &f64, temp_unit: &str) -> String {
    // Rounds an `f64` float to two decimal places and converts the result to
    // a `String`.
    fn round_two_decimals(num: f64) -> String {
        return ((num * 100.0).round() / 100.0).to_string();
    }
    // Converts temperature from fahrenheit to celsius if `temp_unit` is equal
    // to 'F'; else, converts temperature from celsius to fahrenheit.
    let converted_temp = if temp_unit == "F" {
        (temp - 32.0) * 5.0 / 9.0
    } else {
        (temp * 9.0 / 5.0) + 32.0
    };
    // Flips `temp_unit` from 'F' to 'C' and 'C' to 'F'.
    let converted_temp_unit = if temp_unit == "F" { "C" } else { "F" };
    // Returns converted temperature `String`.
    return round_two_decimals(converted_temp) + "˚" + converted_temp_unit;
}
