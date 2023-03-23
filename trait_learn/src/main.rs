mod lib;
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    return if a.len() > b.len() {a} else {b};
}
fn main() {
    let a = "abs";
    let b = String::from("abcdefsdf");
    println!("{}", longer(a, b.as_str()));
}
