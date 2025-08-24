use std::io;

const SUCCESS: bool = true;
const FAILURE: bool = false;

enum TemperatureScale{
    Fahrenheit,
    Celsius,
    Invalid
}

fn main() {

    let (choice, choice_status) = temperature_to_convert();
    let (input, input_status) = get_input();

    if choice_status == FAILURE || input_status == FAILURE {
        println!("invalid inputs, terminating program");
        return;
    }

    println!("the input is {input}");

    let result = match choice {
        TemperatureScale::Fahrenheit => (input - 32.0) * (5.0/9.0),
        TemperatureScale::Celsius => (input * 9.0 / 5.0) + 32.0,
        TemperatureScale::Invalid => {
            println!("how in the fuck did you get here");
            println!("terminating program");
            return;
        }
    };

    println!("the converted temperature is {:.2}", result);
}


fn temperature_to_convert() -> (TemperatureScale, bool){
    println!("Choose which temperature to convert");
    println!("input 1 to convert from fahrenheit to celsius");
    println!("input 2 to convert from celsius to fahrenheit");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: (TemperatureScale, bool) = match choice.trim().parse(){
        Ok(1) => (TemperatureScale::Fahrenheit, SUCCESS),
        Ok(2) => (TemperatureScale::Celsius, SUCCESS),
        Ok(_num) => {
            println!("please input 1 or 2");
            (TemperatureScale::Invalid, FAILURE)
        },
        Err(_) => {
            println!("please input 1 or 2");
            (TemperatureScale::Invalid, FAILURE)
        },
    };

    return choice;
}


fn get_input() -> (f64, bool) {
    println!("please input the temperature");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: (f64, bool) = match input.trim().parse() {
        Ok(num) => (num, SUCCESS),
        Err(_) => {
            println!("please input a number");
            (0.0, FAILURE)
        },
    };

    return input;
}
