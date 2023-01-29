use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
    let i: String = "asdfasdf".to_string();
    let ii: String = String::new("asdfasdf");
}
