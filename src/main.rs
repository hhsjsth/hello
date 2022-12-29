fn main() {
    let array: [String; 8] = core::array::from_fn(|i| String::from("rust is good!"));

    println!("{:#?}", array);
}
