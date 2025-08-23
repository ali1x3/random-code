fn main() {
    println!("Hello, world!");

    another_function();
    parameters(5, 't');
    println!("returned {}", return_type());
}

fn another_function(){
    println!("from another function");
}

fn parameters(x: u32, z: char){
    println!("{} {}", x, z)
}

fn return_type() -> u32 {
    return 5;
    // or 5
    // or return 5
}
