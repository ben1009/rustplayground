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

use rust_init::{hello, iter_files};

fn main() {
    let h = hello::hello();
    println!("{}", h);

    let path = "./_test";
    iter_files::crate_files(path, h.as_bytes()).unwrap();

    let content = iter_files::read_files(path).unwrap();
    println!("{}", content);
}
