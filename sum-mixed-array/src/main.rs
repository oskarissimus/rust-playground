fn main() {
    println!("Hello, world!");
}
use either::Either;

#[allow(dead_code)]
fn sum_mix(arr: &[Either<i32, String>]) -> i32 {
    let mut sum = 0;
    for elem in arr.iter() {
        let num = match elem {
            Either::Left(elem) => *elem,
            Either::Right(elem) => str_to_i32(elem),
        };
        sum += num
    }
    sum
}

fn str_to_i32(s: &String) -> i32 {
    let mut result = 0;
    for (idx, digit_char) in s.chars().rev().enumerate() {
        let digit_int = digit_char.to_digit(10).unwrap() as i32;
        let factor = i32::pow(10, idx as u32);
        result += digit_int * factor;
    }
    result
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::str_to_i32;
    use super::sum_mix;
    use either::Either;

    fn dotest(arr: &[Either<i32, String>], expected: i32) {
        let actual = sum_mix(arr);
        assert!(
            actual == expected,
            "With arr = {arr:?}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn test_str_to_i32() {
        assert!(str_to_i32(&"12".to_string()) == 12)
    }

    #[test]
    fn fixed_tests() {
        dotest(
            &[
                Either::Left(9),
                Either::Left(3),
                Either::Right("17".to_string()),
                Either::Right("3".to_string()),
            ],
            32,
        );
        dotest(
            &[
                Either::Right("5".to_string()),
                Either::Right("0".to_string().to_string()),
                Either::Left(9),
                Either::Left(3),
                Either::Left(2),
                Either::Left(1),
                Either::Right("9".to_string()),
                Either::Left(6),
                Either::Left(7),
            ],
            42,
        );
        dotest(
            &[
                Either::Right("3".to_string()),
                Either::Left(6),
                Either::Left(6),
                Either::Left(0),
                Either::Right("5".to_string()),
                Either::Left(8),
                Either::Left(5),
                Either::Right("6".to_string()),
                Either::Left(2),
                Either::Right("0".to_string()),
            ],
            41,
        );
        dotest(
            &[
                Either::Right("1".to_string()),
                Either::Right("5".to_string()),
                Either::Right("8".to_string()),
                Either::Left(8),
                Either::Left(9),
                Either::Left(9),
                Either::Left(2),
                Either::Right("3".to_string()),
            ],
            45,
        );
        dotest(
            &[
                Either::Left(8),
                Either::Left(0),
                Either::Left(0),
                Either::Left(8),
                Either::Left(5),
                Either::Left(7),
                Either::Left(2),
                Either::Left(3),
                Either::Left(7),
                Either::Left(8),
                Either::Left(6),
                Either::Left(7),
            ],
            61,
        );
    }
}
