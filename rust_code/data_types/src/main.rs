fn main() {
    //integer type can range from 8bits to 128bits
    // u refers to unsigned and i refers to signed
    // integer overflows may occur when incremented beyond the range of the integer
    let x: u8 = 8;
    println!("x is {x}");
    let x: u128 = 19999999;
    println!("x is {x}");

    // floats are all signed, and come in 32 bit or 64 bit.
    // both have roughly the same speed in terms of computation
    let x: f32 = 0.5;
    println!("this is 32 bit float {x}");

    let x: f64 = 0.24923747239;
    println!("this is 64 bit float {x}");

    // testing how division works on integer and float
    
    let x = 5;
    let y = 3;
    let z = x / y;
    println!("x / y is {z}");

    let _x = 8;
    let _y = 3.0;
    // let z = x / y;    <- this does not work
    println!("x / y is {z}");

    let x = 8.2;
    let _y = x % 2.5;


    // chars are always single quotes
    let _x: char = 'z';


    // tuples have a fixed length and cannot be changed
    // tuples can be composed of different data types
    // we can access the components by using the . operator
    // the index in rust start at 0 
    let tup: (u8, i8, char) = (8, -1, 'z');
    let (x, y, z) = tup;
    println!("x, y and z is {}, {}, {}", x, y, z);
    println!("x, y and z is {}, {}, {}", tup.0, tup.1, tup.2);

    // arrays also have a fixed length in rust
    // arrays contain multiple values of the same type
    let x = [1, 2, 3, 4, 5];
    println!("x is {}", x[0]);
}
