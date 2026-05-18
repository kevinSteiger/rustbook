fn main() {
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(5);
    v3.push(6);
    v3.push(7);
    v3.push(8);

    let mut v4 = vec![1,2,3,4,5];
    let third: &i32 = &v4[2];
    println!("The third element is: {}", third);


    let third: Option<&i32> = v4.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There's no third element"),
    }


    let first = &v4[0];
    println!("The first element is: {first}");
    v4.push(6);

}
