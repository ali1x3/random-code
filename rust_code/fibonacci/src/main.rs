use std::io;

fn main() {
    println!("please input the nth fibonacci number to calculate");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_num) => println!("the read input is {}", input),
        Err(e) => eprintln!("an error has occured: {}", e),
    }

    let input: u8 = match input.trim().parse() {
        Ok(num) => {
            println!("The input is vaild");
            num
        },
        Err(e) => {
            println!("The input is invaild, please check if input is integer from 0 to 255");
            eprintln!("Error: {}", e);
            return;
        }
    };

    let result = fibbonacci(input);
    println!("the nth fibobonacci number is {}", result);
}


fn fibbonacci (input: u8) -> u128{
    match input {
        1 => 1,
        0 => 0,
        num => fibbonacci(num - 1) + fibbonacci(num - 2)
    }
}
