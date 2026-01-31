fn main() {
    println!("Hello, world!");

    print_loops();
}


fn print_loops(){
    for number in 1..(12 + 1) {
        println!("On day {number}:");

        for value in 1..(number + 1){
            println!("value {value}");
        }
    }
}