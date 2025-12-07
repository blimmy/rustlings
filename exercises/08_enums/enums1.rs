#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize,
    Move,
    Echo,
    ChangeColor,
    Quit,
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}


/*
    how to declare enum
    enum = type ที่บอกว่า “ค่าหนึ่ง ๆ จะเป็นได้ แค่หนึ่งในหลายรูปแบบที่กำหนดไว้”
 */