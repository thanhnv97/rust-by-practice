fn main() {
    let mut s: String = "hello, ";
    s.push_str("world");
    s.push("!");

    move_ownership(s);

    assert_eq!(s, "hello, world!");

    println!("Success!");
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}