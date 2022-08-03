// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
   Quit,
   Echo,
   Move,
   ChangeColor,

}

// Structs look like this 
// enum Message {
//     Quit: String::from("Quit cunt"),
//     Echo: String::from("Echo cunt"),
//     Move: String::from("Move cunt"),
//     ChangeColor: String::from("ChangeColor cunt"),
 
//  }

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
