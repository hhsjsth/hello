fn main() {
    let mut s1 = String::from("hello,");
    let s2 = String::from("world!");
    // 在下句中，s1的所有权被转移走了，因此后面不能再使用s1
    s1 = s1 + &s2;
    // 下面的语句如果去掉注释，就会报错
    println!("{}", s1);
}
