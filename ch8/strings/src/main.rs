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

}
