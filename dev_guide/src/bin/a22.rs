// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_concat() {
        let expected: String = String::from("pit trak");
        let result: String = concat("pit", "trak");

        assert_eq!(expected, result);
    }

    #[test]
    fn test_div() {
        struct TestCase {
            a: i32,
            b: i32,
            expected: Option<i32>,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                a: 10,
                b: 2,
                expected: Some(5),
            },
            TestCase {
                a: 9,
                b: 3,
                expected: Some(3),
            },
            TestCase {
                a: 1,
                b: 0,
                expected: None,
            },
        ];

        for t in test_cases {
            let result: Option<i32> = div(t.a, t.b);
            assert_eq!(t.expected, result);
        }
    }

    #[test]
    fn test_clamp() {
        struct TestCase {
            n: i32,
            upper: i32,
            lower: i32,
            expected: i32,
        }

        let test_cases: Vec<TestCase> = vec![
            TestCase {
                n: 9,
                lower: 5,
                upper: 7,
                expected: 7,
            },
            TestCase {
                n: 3,
                lower: 5,
                upper: 7,
                expected: 5,
            },
        ];

        for t in test_cases {
            let result: i32 = clamp(t.n, t.lower, t.upper);
            assert_eq!(t.expected, result);
        }
    }
}
