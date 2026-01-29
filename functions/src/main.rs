fn main() {
    println!("Hello, world!");

    another_function(5, '$');

    let x = plus_one(5);
    println!("The value of x is: {}", x)
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {unit_label}{x}");
}


fn plus_one(x: i32) -> i32{
    x + 1
}