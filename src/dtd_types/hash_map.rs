use std::collections::HashMap;

pub fn test_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("hash map scores : {:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("hash map scores : {:?}", scores);

    let mut x = teams.iter();
    let mut v = x.next();

    while let Some(s) = v {
        println!("v is {:?}", s);
        v = x.next();
    }


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    // owner of field_name, filed_value become to 'map',
    //   can't access them after following expression
    map.insert(field_name, field_value);

    // following code will get error:
    // println!("filed_name is {}", field_name)
    // |                                  ^^^^^^^^^^ value borrowed here after move

    let name = "Favorite color".to_string();
    if let Some(x) = map.get(&name) {
        println!("get filed({}) value is {}", name, x);
    }
}