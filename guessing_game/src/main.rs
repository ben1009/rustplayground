use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess game, the code will end if guess is equal, otherwise keep going !");
    println!("please input your guess");
    loop {
        let num = rand::thread_rng().gen_range(1..=100);
        println!("the num:{}", num);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("fail to readline");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{guess} is not a num, err is {}", err);
                continue;
            }
        };

        println!("your guess: {}", guess);

        match guess.cmp(&num) {
            Ordering::Less => println!("guess: {} is less than num {}", guess, num),
            Ordering::Greater => println!("guess: {} is greater than num {}", guess, num),
            Ordering::Equal => {
                println!("guess: {} is equal than num {}", guess, num);
                break;
            }
        }
    }
}
