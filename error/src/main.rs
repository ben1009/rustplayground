use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main() {
    let ret = test();
    println!("{:?}", ret);
}

fn t() -> Result<String, Error> {
    return Ok("ss".to_string());
}
fn test() -> Result<String, Error> {
    // enum magic !!!
    return t();
    // ? only return when error happened in the caller,
    // the block below can return Ok(String) or Err(Error), as long as the Type is Result,
    // and the type match the type constrains T, E
    let mut file_open = File::open("test")?;
    let mut str = String::new();
    file_open
        .read_to_string(&mut str)?
        .checked_add(1)
        .ok_or(Error::new(ErrorKind::AddrInUse, "test"))?;

    return Err(Error::new(ErrorKind::AddrInUse, "test"));
}
