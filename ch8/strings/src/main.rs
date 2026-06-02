fn main() {
    let data = "initial contents";
    let s = data.to_string();
        
    let mut s = String::from("foo");
    let s2 = String::from("bar");

    s.push_str(&s2);
    s.push('s');
    println!("{s2}");


    let s1 = String::from("Hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2;


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    let s1 = String::from("hi");
    let h = &s1[0..1];
    println!("{h}");

    let hello = "Здравствуйте";
    let answer = &hello[0..4];
    println!("{answer}");

    for c in hello.bytes() {
        println!("{c}");
    }
}
