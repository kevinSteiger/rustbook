//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Value of x inner scope: {}", x);
    }
    println!("The value of x outer scope is: {}", x);

    let quotient = 56.7 / 32.2;
    println!("{quotient}");

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Here's a tup: {}, {}, {}", x, y, z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    

}
