struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rectangle = Rectangle{
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {}",
        area(&rectangle)
    );
}

fn area (rect: &Rectangle) -> u32 {
    rect.height * rect.width
}
