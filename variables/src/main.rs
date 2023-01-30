fn main() {
    let mut i = 1;
    while i < 10 {
        let ret = fib(i);
        println!("i={i}, ret is {ret}");
        i += 1; // not support i++ https://doc.rust-lang.org/1.2.0/complement-design-faq.html#why-no---x-or-x++?
    }
}

fn fib(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    return fib(n - 1) + fib(n - 2);
}
