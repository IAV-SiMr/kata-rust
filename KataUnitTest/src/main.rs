// Start you Kata here
fn adder(a: i64, b: i64) -> i64 {
    return a + b;
}



#[cfg(test)]
mod tests {
    use super::*;

    // Tests should be written below
    #[test]
    fn test_adder() {
        assert_eq!(adder(2, 2), 4); 
    }
}