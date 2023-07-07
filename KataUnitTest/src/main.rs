// src/main.rs
// -------------

fn adder(a: i64, b: i64) -> i64 {
    return a + b;
}



// src/main.rs
// -------------

// ......

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
        assert_eq!(adder(2, 2), 5); 
    }
}