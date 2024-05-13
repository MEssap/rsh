#![no_std]

extern crate alloc;

use alloc::string::String;

fn main() {
    let mut input = String::new();
    // stdin().read_line(&mut input).unwrap();

    // read_line leaves a trailing newline, which trim removes
    // read_line 会在最后留下一个换行符，在处理用户的输入后会被删除
    let command = input.trim();

    // Command::new(command).spawn().unwrap();
}
