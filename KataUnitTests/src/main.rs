// src/main.rs
// -------------

fn adder(a: i64, b: i64) -> i64 {
    return a + b;
}

fn my_panic() {
    panic!("oops...");
}

fn substract() -> i64 {
    // TODO: logic not yet implemented
    return 4;
}



// src/main.rs
// -------------

// ......

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adder() {
        assert_eq!(adder(2, 2), 4);
    }

    #[test]
    #[should_panic(expected = "oops...")]
    fn test_my_panic() {
        my_panic()
    }

    #[test]
    #[ignore]
    fn test_substract() {
        // This test is ignored for now
        assert_eq!(substract(), 6);
    }
}