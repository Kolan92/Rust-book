fn main() {
    println!("Hello, world!");

    let mut s1 = String::from("hello");

    let len1 = calculate_length(&mut s1);
    let len2 = calculate_length(&mut s1);

    println!("The length of '{}' is {}.", s1, len1 + len2);
    let s = String::from("hello world");

    let word = first_word(&s);
    println!("{}", word);

    let mut s = String::from("hello");

    let s2: &mut String = &mut s;

    s2.push('1');
  
    let s3: &str = &s;


}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
