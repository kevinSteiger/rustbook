enum IpAddrKind{
    v4(u8, u8, u8, u8),
    v6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) -> String{
        match self{
            Message::Quit => String::from("Quit"),
            Message::Move{x, y} => format!("Move Message contains: X: {}, Y: {}", x, y),
            Message::Write(text) => format!("Write Message contains: {}", text),
            Message::ChangeColor(r, g, b) => format!("ChangeColor Message contains: {}, {}, {}", r, g, b)

        }
    }
}


fn main() {
    let home = IpAddrKind::v4(127,0,0,1);
    let loopback = IpAddrKind::v6(String::from("::1"));

    let m1 = Message::Write(String::from("yo"));

    let out = m1.call();

    println!("{}", out);

    println!("Hello, world!");
}
