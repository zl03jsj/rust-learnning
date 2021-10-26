https://kaisery.github.io/trpl-zh-cn/ch01-00-getting-started.html

一些rust术语:
rustaceaa:
Rustacean（复试形式为 Rustaceans）中文意思为 Rust 开发者，Rust 用户，Rust 爱好者，本词一般不译，直接取原文。
Rustacean 来源于单词 crustacean，后者意思为：甲壳纲动物（如蟹、龙虾）。
注意 Rust 开发者不要写成 Ruster，另外 Rustacean 一般第一个字母为大写形式，就和 Rust 一样。

cargo和rust的关系:
cargo是rust的构建系统和包管理器.cargo用于项目管理, rustc是编译器.

编译单个文件:
rustc filepath.rs

cargo创建项目:
cargo new hello_cargo
cargo编译项目:
cargo build
编译并运行:
cargo run
检查是否项目可以编译:
cargo check
编译发布, 生成的可执行经过编译优化,通常执行更快:
cargo build --release
cargo更新:
cargo update,会自动检查最新版本并更新
在项目下运行下面的命令, 可以查看项目中依赖的所有carte的document:
cargo doc --open

rust中的包术语为:crate,
crate分为包crate和二进制crate,
    包crate: 库, 被其它rust程序调用.
    二进制crate: 可执行程序, 直接运行.

查看项目目录详情:
tree -L 3
.
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug
        ├── build
        ├── deps
        ├── examples
        ├── hello_cargo
        ├── hello_cargo.d
        └── incremental

rust的宏调用, 表达式后面为'!'

rust变量默认为immutable,不可变.

let apples = 5; // immutable
let mut bananas = 5; // mutable

函数引用作为输入参数, 并修改引用变量的值:
let mut guess = String::new();
io::stdin()
.read_line(&mut guess)
.expect("Failed to read line");

代码分析,
定义可变参数'guss'
获取io包中的Stdin实例对象
调用stdin实例对象的resd_line,尝试从命令行读取一行数据并追加到guess的末尾, read_line的返回值是一个io::Result的实例.(io::Result是enum::Result的特化版本)
    enum::Result包含两种类型:Err和Ok
expect内部判断result是Err还是Ok(关于Ok和Err进一步需要了解泛型),
    如果是Err会导致panic.

字符串打印参数的编写方式为:
println!("x = {} and y = {}", x, y);

如果需要依赖新的crate, 以rand为例:
修改Cargo.toml文件, 在[dependencies]section添加:
rand = "0.8.3"
其中"0.8.3"是"^0.8.3"的简写, 为名称为rand的crate的与"0.8.3"的api兼容的版本.
在使用cargo build编译的时候,cargo会自动下载相关的依赖项.
cargo build编译后会生成'Cargo.lock',用于锁定当前项目编译时使用的各种crate的版本.

在rust中, 指定类型:
let guess: u32 = guess.trim().parse().expect("Please type a number!");

rust中的常量, 不可变变量, 和可变变量:
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
let x = 1;
let x mul = 1;

隐藏shadowing:
let x = 1; // 直接使用 x = 2; 会出错, x是不可变变量, 但可以隐藏
let x: uint32 = 100;

let mul 和 let 的区别是: 
let从新创建了对象, mul修改.

定义数组的几种方式:
let a = [1, 2, 3, 4, 5];
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5]; // let a = [3,3,3,3,3];

rust中的,代码块的值,是最后一个表达式的值.表达式的使用技巧:
1. let x = 5;
   let y = { let x = 3; x + 1 };
2. let number = if condition { 5 } else { 6 };

rust循环和标签:
1: 'exit_label':loop {
    loop{
        break 'exit_label';
    }
}

2: let x = loop {
    break 10;
} // 得到x为10.

3: let a = [10, 20, 30, 40, 50];
   for element in a.iter() {
       println!("the value is: {}", element);
   }

4: for number in (1..4).rev() {
        println!("{}!", number);
   }

所有权:
Rust 中的每一个值都有一个被称为其 所有者(owner）的变量。
值在任一时刻有且只有一个所有者。
当所有者（变量）离开作用域，这个值将被丢弃。

rust中既没有GC(垃圾回收)也不用人工释放alloca的内存, 就是通过类型的drop, 和所有权变换实现的.
所以下面的代码会报错:
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}, world!", s1);
在第二行赋值后,字符串实例的所有权就会从s1变成s2,
s1不在有效,在程序执行到变量作用域之外时, 不会调用其drop.

引用和可变引用
引用默认是不可变的, 可以把函数参数定义为可变引用,这样就可以在函数中修改类型,
并影响函数之外的这个变量的值.
但在作用区域内, 每个实例只能定义一个可变引用.
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

但下面的方式不会有问题, 这没有违反规则:
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
let r2 = &mut s;

下面的代码会编译失败,对于r1, r2来说, 他是不可变的, 但r3却可能改变其值,
所以这是矛盾的, 编译器不会允许发生.
let mut s = String::from("hello");
let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题
println!("{}, {}, and {}", r1, r2, r3);

但r1,r2的作用域仅仅从声明到最后一次使用它, 所以下面的代码还是没有问题的:
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s; 
println!("{}, {}, and {}", r1, r2, r3);
let r3 = &mut s;

在任意给定时间,要么只能有一个可变引用,要么只能有多个不可变引用.引用必须总是有效的.

构造结构体的简写方法,变量名和结构体中的字段名一样就.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

结构体更新语法:
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
上面的代码,设置email和username,其它字段的值都等于usesr1中的对应字段的值.

元祖结构体:
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

结构体成员是引用的话, 必须指定其生命周期.



在启动Rust程序时, 添加环境变量:
RUST_BACKTRACE=1
在程序崩溃的时候, 会打印调用堆栈,和更详细的错误信息,
需要debug标识.没有添加--release是自动包含的.



rust 函数返回引用的问题:
参考资料:https://colobu.com/2019/08/13/strategies-for-returning-references-in-rust/

由于引用的对象是在scope中创建, 在脱离scope时, 对象就被释放了.
所以引用是无效的引用,rust编译器会出错.
需要使用Box?在堆上分配.返回box对象.

rust 函数参数和返回值生命周期的问题:
规则:
1. 函数引用类型参数有各自的生命周期
2. 只有唯一一个引用类型参数,则返回值的生命周期就是这个参数的.
3. 参数中带有&self, 则返回值的生命周期和&self一样.
由于引用的生命周期是在编译时就需要确定的(不是在运行时确定的),
所以对于类似的函数与:
fn longest_str(s1 :&str, s2 :&str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
函数在编译时就需要确定返回值引用的生命周期用于错误的检查.
但是,由于返回值有可能是s1, 也有可能是s2, 并且
s1和s2的生命周期有可能是不同的,编译器没办法确定返回值的生命周期是什么..
就会出现编译错误.

rust 运行测试的几种方式:
cargo test --package hello_cargo --lib tests::run -- --test-threads=1 --nocapture 
cargo test run
cargo test --lib tests
cargo test test_add
运行标记为#[ignore]的测试:
cargo test --lib tests -- --ignored

测试模块的 #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码
#[cfg(test)]
mod tests {}
