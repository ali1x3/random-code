use std::io;

const FAHRENHEIHT: u8 = 1;
const CELSIUS: u8 = 2;
const SUCCESS: bool = true;
const FAILURE: bool = false;

fn main() {

    let (choice, choice_status) = temperature_to_convert();
    let (input, input_status) = get_input();

    if choice_status == FAILURE || input_status == FAILURE {
        println!("invalid inputs, terminating program");
        return;
    }

    println!("the input is {input}");

    let result = match choice {
        FAHRENHEIHT => (input - 32.0) * (5.0/9.0),
        CELSIUS => (input * 9.0 / 5.0) + 32.0,
        _ => {
            println!("how in the fuck did you get here");
            println!("terminating program");
            return;
        }
    };

    println!("the converted temperature is {result}");
}


fn temperature_to_convert() -> (u8, bool){
    println!("Choose which temperature to convert");
    println!("input 1 to convert from fahrenheit to celsius");
    println!("input 2 to convert from celsius to fahrenheit");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: (u8, bool) = match choice.trim().parse(){
        Ok(1) => (FAHRENHEIHT, SUCCESS),
        Ok(2) => (CELSIUS, SUCCESS),
        Ok(_num) => {
            println!("please input 1 or 2");
            (0, FAILURE)
        },
        Err(_) => {
            println!("please input 1 or 2");
            (0, FAILURE)
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
