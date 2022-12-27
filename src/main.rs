// 修复错误
#[derive(Debug)]
struct File {
    name: String,
    data: String,
}
fn main() {
    let f = File {
        name: String::from("readme.md"),
        data: "Rust By Practice".to_string(),
    };

    let ref _name = &f.name;

    // 只能修改这一行
    println!("{}, {}, {:?}", f.name, f.data, f);
}
