#![allow(unused_imports)]

use std::{env, fs, process, io, error::Error};

pub mod mini_grep;

use mini_grep::{search, Config, run};

fn main() {

    // We start with an Option value (Option<i32> in this case).
    let some_number = Some(9);

    // 让我们用我们的数字做一些连续的计算。
    // 这里的关键点是我们不必拆开包装
    // 选项类型的内容-相反，我们只是
    //改变其内容。整个操作的结果
    // will still be an Option<i32>. If the initial value of
    // 'some_number'是'None'而不是9，则结果
    //  也将是“无”。
    let another_number = some_number
        .map(|n| n - 1) // => Some(8)
        .map(|n| n * n) // => Some(64)
        .and_then(|n| divide(n, 4)); // => Some(16)

    // 在上面的最后一行，我们正在使用助手进行除法
    // 功能（定义：请参阅底部）。
    // “ and_then”与“ map”非常相似，但允许我们传递
    //该函数本身返回Option类型。为了确保我们
    // don't end up with Option<Option<i32>>, 'and_then' flattens the
    // 结果（在其他语言中，“ and_then”也称为“ flatmap”）。

    println!("{}", to_message(another_number));
    // => "16绝对是个数字！"

    // 为了完整起见，让我们检查一下结果
    // 除以零。
    let final_number = another_number
        .and_then(|n| divide(n, 0)); // => None

    println!("{}", to_message(final_number));
    // => "None!"
}

//只是整数除法的辅助函数。以防万一
// 除数为零，结果为“无”。
fn divide(number: i32, divisor: i32) -> Option<i32> {
    if divisor != 0  { Some(number/divisor) } else { None }
}

// 创建一条消息，告诉我们是否
// Option<i32> contains a number or not. There are other
// 达到相同结果的方法，但让我们使用
// 再次映射！
fn to_message(number: Option<i32>) -> String {
    number
        .map(|n| format!("{} is definitely a number!", n)) // => Some("...")
        .unwrap_or("None!".to_string()) // => "..."
}
// fn main() {
//     let blocks :&[&[u8]] =&[&[1,2,3], &[3, 4, 5]];
//
//     let args: Vec<String> = env::args().collect();
//
//     let config: Config = Config::new(&args).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });
//
//     println!("Searching for {}", &config.query);
//     println!("In file {}", &config.filename);
//
//     if let Err(e) = run(config) {
//         println!("Application error: {}", e);
//         process::exit(1);
//     }
// }
//
