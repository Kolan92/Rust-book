use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    for x in &v {
        println!("{x}");
    }
    v.push(5);

    println!("Hello, world!");

    let eight = &v.get(8);

    if let Some(value) = eight {
        println!("{value}")
    } else {
        println!("none")
    }

    let hello = String::from("Здравствуйте");

    println!("{}", hello.len());

    for c in "Зд".chars() {
        println!("{}", c);
    }
    for b in "Зд".bytes() {
        println!("{}", b);
    }

    let mut scores = HashMap::new();

    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    let v = scores.get("Blue");

    if let Some(v) = v {
        println!("{v}")
    }

    scores.insert("Blue", 666);

    let v = scores.get("Blue");

    if let Some(v) = v {
        println!("{v}")
    }


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
