#![allow(dead_code)]

enum IpAddrStr {
    V4(String),
    V6(String),
}

enum IpAddrMul {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct QuitMessage;

struct MoveMessage {
    x: i32,
    y: i32,
}

struct WriteMessage(String);

struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {}
}

pub fn test_call() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

pub fn test_option() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
}

pub fn test_ip() {
    let _home = IpAddrStr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrStr::V6(String::from("::1"));

    let _home = IpAddrMul::V4(127, 0, 0, 1);
    let _loopback = IpAddrStr::V6(String::from("::1"));
}