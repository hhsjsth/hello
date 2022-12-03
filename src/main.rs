fn main() {
    let mut s = String::from("hello world");

    s.clear(); // error!

    let word = first_word(&s);

    println!("the first word is: {}", word);
}
fn first_word(s: &String) -> &str {
    &s[..1]
}
