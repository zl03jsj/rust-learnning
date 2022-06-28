#![allow(dead_code)]
#![allow(unused_doc_comments)]

const INT: u64 = 1 << 11;
const STRING: u64 = 1 << 12;

#[macro_export(local_inner_macros)]
macro_rules! hi {
    ($size:expr, $f:ident) => {
        hi!($size, $f,)
    };
    ($size:expr, $f:ident, $($args:expr,)*) => {
        $f($($args),*)
    }
}

fn add<A: std::ops::Add<B, Output=A>, B>(a: A, b: B) -> A {
    a + b
}

pub fn use_hi_macro() {
    let a = 10;
    let b = 20;
    let x = add::<i32, i32>(a, b);

    println!("number add, {} + {}  = {}", a, b, x);

    let a = "hello".to_string();
    let b = " world!";

    /// rustc --cfg 'use_hi="no"' ./src/main.rs
    /// ```
    /// if cfg!(use_hi = "no") {
    ///     let x = add::<String, &str>(a.clone(), b);
    ///     println!("string add, {} + {} = {}", a, b, x);
    /// } else {
    ///     println!("------------------------use macro hi.------------------------");
    ///     let x = hi!(STRING, add, a.clone(), b, );
    ///     println!("string add, {} + {} = {}", a, b, x);
    /// }
    /// ```

    // feature, 是在`cargo.toml`中添加[features]项, 在编译时, cargo会将
    // --cfg 'feature="no_hi"' 传给 rustc 作为编译项,
    // cargo run --features no_hi
    if cfg!(feature="no_hi") {
        let x = add::<String, &str>(a.clone(), b);
        println!("string add, {} + {} = {}", a, b, x);
    } else {
        println!("------------------------use macro hi.------------------------");
        let x = hi!(STRING, add, a.clone(), b, );
        println!("string add, {} + {} = {}", a, b, x);
    }
}
