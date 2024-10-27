fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    let previous_gray = gray(n - 1);
    let mut result = Vec::with_capacity(2usize.pow(n as u32));

    // префікс '0' до першої половини
    for code in &previous_gray {
        result.push(format!("0{}", code));
    }

    // префікс '1' до другої половини (у зворотному порядку)
    for code in previous_gray.iter().rev() {
        result.push(format!("1{}", code));
    }

    result
}

#[test]
fn test() {
    let test_data = [
        (0, vec![""]),
        (1, vec!["0", "1"]),
        (2, vec!["00", "01", "11", "10"]),
        (3, vec!["000", "001", "011", "010", "110", "111", "101", "100"]),
        (4, vec![
            "0000", "0001", "0011", "0010", "0110", "0111", "0101", "0100",
            "1100", "1101", "1111", "1110", "1010", "1011", "1001", "1000",
        ]),
    ];

    for (n, expected) in test_data {
        assert_eq!(gray(n), expected);
    }
}
