fn main() {
    let mut s = String::from("hello world");
    s.push_str(", you!");
    let first = first_word(&s);
    println!("{}", first);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
