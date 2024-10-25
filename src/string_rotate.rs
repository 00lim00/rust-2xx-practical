fn rotate(s: String, n: isize) -> String {
    let len = s.len();

    // Нормалізуємо n, щоб бути впевненими, що воно в межах довжини рядка
    let n = n.rem_euclid(len as isize) as usize; // Використовуємо rem_euclid для позитивного зсуву

    // Розділяємо рядок на дві частини та об'єднуємо їх у зворотному порядку
    let (left, right) = s.split_at(len - n);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.to_string(), *n), exp.to_string());
        });
    }
}
