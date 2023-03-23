pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test2() {
        panic!("Make it fall");
    }
}
 