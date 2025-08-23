fn main() {
    // variables are immutable by default
    let mut x: u8 = 5;
    println!("x is {x}");
    x = 8;
    println!("x is {x}");


    // const are usually named in caps lock
    const THIS_IS_A_CONST: u32 = 30;
    println!("This is a const: {THIS_IS_A_CONST}");


    // shadowing I still dont understand this much
    // this allows for retyping a variable
    // this is not possible with mut
    let x = 10;
    {
        let x = 15;
        println!("x is {x}");
    }
    println!("x is {x}");
}
