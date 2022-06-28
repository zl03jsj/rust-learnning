use std::vec;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


pub fn test_vector() {
    let mut v: vec::Vec<i32> = vec::Vec::new();
    v.push(10);
    v.push(11);
    v.push(12);

    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let num: usize = 100;

    match v.get(num) {
        Some(third) => println!("The {} element is {}", num, third),
        None => println!("There is no {} element.", num),
    }

    print_vector(&v);

    change_vector(&mut v);

    print_vector(&v);



    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn change_vector(v: &mut vec::Vec<i32>) {
    for i in v {
        *i += 1;
    }
}

fn print_vector(v: &vec::Vec<i32>) {
    println!("----------------------------");
    for i in v {
        println!("{}", i)
    }
}