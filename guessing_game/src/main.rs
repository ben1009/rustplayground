use std::io;

fn main() {
    println!("guess game");
    println!("please input your guess");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("fail to readline");
    println!("your guess: {}", guess)
}
