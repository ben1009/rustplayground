// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use std::time::{Duration, SystemTime};

use await_tree::{Config, InstrumentAwait, Registry};
use tokio::{
    fs::{File, OpenOptions},
    io::{self, AsyncReadExt, AsyncWriteExt},
};

#[tokio::main]
async fn main() {
    let registry = Registry::new(Config::default());
    let root = registry.register((), "foo");
    tokio::spawn(root.instrument(test()));

    loop {
        tokio::time::interval(Duration::from_millis(500))
            .tick()
            .await;
        if let Some(t) = registry.get(()) {
            println!("{t}");
        } else {
            println!("None");
            break;
        }
    }
}

async fn read_from_file(file_name: &str) -> io::Result<Vec<u8>> {
    let mut f = File::open(file_name).instrument_await("File::open").await?;
    let mut str = vec![];
    f.read_to_end(&mut str)
        .instrument_await("read_to_end")
        .await?;

    Ok(str)
}

async fn write_to_file(file_name: &str) -> io::Result<()> {
    // pending::<()>().instrument_await("pending in baz").await;
    tokio::time::sleep(Duration::from_millis(100))
        .instrument_await("sleep")
        .await;

    let mut f = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_name)
        .instrument_await("OpenOptions::new")
        .await?;
    let s = format!(
        "Hello World, {}\n",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );

    let _ = f.write(s.as_bytes()).instrument_await("write").await?;

    f.sync_all().instrument_await("sync_all").await
}

async fn test() -> io::Result<()> {
    tokio::time::sleep(Duration::from_millis(500))
        .instrument_await("sleep")
        .await;

    let file_name = "test.txt";
    write_to_file(file_name)
        .instrument_await("write_to_file, test.txt")
        .await?;

    let ret = read_from_file(file_name)
        .instrument_await("read_from_file, test.txt")
        .await
        .unwrap_or_default();
    println!(
        "{}, {}",
        file_name,
        String::from_utf8(ret).unwrap_or_default()
    );

    let file_name = "test1.txt";
    write_to_file(file_name)
        .instrument_await("write_to_file, test1.txt")
        .await?;

    let ret = read_from_file(file_name)
        .instrument_await("read_from_file, test1.txt")
        .await
        .unwrap_or_default();
    println!(
        "{}, {}",
        file_name,
        String::from_utf8(ret).unwrap_or_default()
    );

    let file_name = "test2.txt";
    write_to_file(file_name)
        .instrument_await("write_to_file, test2.txt")
        .await?;

    let ret = read_from_file(file_name)
        .instrument_await("read_from_file, test2.txt")
        .await
        .unwrap_or_default();
    println!(
        "{}, {}",
        file_name,
        String::from_utf8(ret).unwrap_or_default()
    );

    let file_name = "test3.txt";
    write_to_file(file_name)
        .instrument_await("write_to_file, test3.txt")
        .await?;

    let ret = read_from_file(file_name)
        .instrument_await("read_from_file, test3.txt")
        .await
        .unwrap_or_default();
    println!(
        "{}, {}",
        file_name,
        String::from_utf8(ret).unwrap_or_default()
    );

    let file_name = "test4.txt";
    write_to_file(file_name)
        .instrument_await("write_to_file, test4.txt")
        .await?;

    let ret = read_from_file(file_name)
        .instrument_await("read_from_file, test4.txt")
        .await
        .unwrap_or_default();
    println!(
        "{}, {}",
        file_name,
        String::from_utf8(ret).unwrap_or_default()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
