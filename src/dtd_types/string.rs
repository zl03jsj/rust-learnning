pub fn test_string() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let sx = "".to_string() + &s1[..] + "-" + &s2[..] + "-" + &s3[..];
    println!("s is : {}", sx);
    let sx = "".to_string() + &s1 + "-" + &s2 + "-" + &s3;
    println!("s is : {}", sx);

    let hello: String = String::from("Здравствуйте");
    
    let s: &str = &hello[0..4];
    println!("s is {}", s);


    let s = &hello[..];
    println!("s[0] is {}", s);
    let s = &hello[..];
    println!("s[1] is {}", s);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{}", c)
    }
}