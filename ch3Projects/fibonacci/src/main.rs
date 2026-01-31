fn main() {
    println!("Hello, world!");

    let n = 8;
    let fib = nth_fibonacci(n);
    println!("{}th fibonacci sum: {}", n, fib);
}


fn nth_fibonacci(n: i32) -> i32{
    let mut prev:i32 = 0;
    let mut curr:i32 = 1;

    if n == 1{
        return prev;
    }
    else if n == 2{
        return curr;
    }
    else{
        for num in 2..(n){
        println!("curr iteration: {num} sum: {curr}");

        let old_curr = curr;
        curr = prev + curr;

        prev = old_curr;
    }
    }
    

    curr
}