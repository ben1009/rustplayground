const X_1: i64 = 122;
fn main() {
    println!("x is {X_1}");
    let x = 1;
    println!("x is {x}");

    let x = 5;
    println!("x is {x}");

    let x = "fsdff";

    {
        let x = 7;
        println!("x is {x}");
    }
    println!("x is {x}");
}
