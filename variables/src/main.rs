//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Value of x inner scope: {}", x);
    }
    println!("The value of x outer scope is: {}", x);

}
