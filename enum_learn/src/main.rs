enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
impl IpAddrKind {
    fn call(&self) {
        match self {
            IpAddrKind::V4(a, b, c, d) => println!("{}.{}.{}.{}", a, b, c, d),
            IpAddrKind::V6(s) => println!("{}", s),
        }
    }
}
fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));
    four.call();
    six.call();
}
