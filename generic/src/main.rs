fn main() {
    let t = Test { x: 111, y: "sdfsf" };
    println!("{}", t.print());
}

pub trait Print {
    fn print(&self) -> String;
}

struct Test<'a, T> {
    x: T,
    y: &'a str,
}

impl<T: ToString> Print for Test<'_, T> {
    fn print(&self) -> String {
        let ret = &mut (self.y.to_string());
        ret.push_str(&self.x.to_string());
        return ret.to_string();
    }
}
