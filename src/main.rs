enum MyEnum {
    Foo,
    Bar,
}

fn main() {
    let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];

    v.iter().filter(|x| x == MyEnum::Foo); // 编译错误

    v.iter().filter(|x| matches!(x, MyEnum::Foo)); // 通过编译
}
