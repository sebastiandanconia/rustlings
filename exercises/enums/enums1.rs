// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Echo(String),
    Move(i16, i16),
    Quit,
    ChangeColor{r: u8, g: u8, b: u8},
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("A-OK")));
    println!("{:?}", Message::Move(10, 30));
    println!("{:?}", Message::ChangeColor{r: 0, g: 128, b: 192});
}
