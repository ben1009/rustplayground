fn main() {
    let mut s1 = "111".to_string();
    let s2 = "2".to_string();
    s1.push_str(&s2);
    println!("{:?}", s1);
    for c in s1.bytes() {
        println!("{c}");
    }
}
