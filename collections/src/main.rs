fn main() {
    let mut v = vec![1, 2, 3, 4];
    for i in &mut v {
        *i += 1; // boxed ? why the value in vec changed here
    }

    println!("v:{:#?}", v)
}
