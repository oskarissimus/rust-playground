fn main() {
    println!("Hello, world!");
}
#[allow(dead_code)]
fn fake_bin_loop(s: &str) -> String {
    let mut output = String::new();
    for digit in s.chars() {
        if digit < '5' {
            output.push('0');
        } else {
            output.push('1')
        }
    }
    output
}
#[allow(dead_code)]
fn fake_bin(s: &str) -> String {
    String::from_iter(s.chars().map(|x| if x < '5' { '0' } else { '1' }))
}

// https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html
#[cfg(test)]
mod tests {
    use super::fake_bin;

    #[test]
    fn basic_tests() {
        assert_eq!(fake_bin("45385593107843568"), "01011110001100111");
        assert_eq!(fake_bin("509321967506747"), "101000111101101");
        assert_eq!(
            fake_bin("366058562030849490134388085"),
            "011011110000101010000011011"
        );
        assert_eq!(fake_bin("15889923"), "01111100");
        assert_eq!(fake_bin("800857237867"), "100111001111");
    }
}
