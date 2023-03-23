fn main() {
    let mut s1 = String::from("hello");
    test(&mut s1);
    println!("{s1}");
}
fn test(s: &mut String) {
    s.push_str(", world");
}
