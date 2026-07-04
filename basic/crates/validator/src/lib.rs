pub fn validate_cpf(cpf: &str) -> bool {
    let cpf_digits: Vec<u32> = cpf.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    if cpf_digits.len() != 11 {
        return false;
    }

    let sum1: u32 = cpf_digits.iter().take(9).enumerate()
        .map(|(i, &digit)| digit * (10 - i as u32))
        .sum();

    let check_digit1 = (sum1 * 10) % 11;
    let check_digit1 = if check_digit1 == 10 { 0 } else { check_digit1 };

    if check_digit1 != cpf_digits[9] {
        return false;
    }

    let sum2: u32 = cpf_digits.iter().take(10).enumerate()
        .map(|(i, &digit)| digit * (11 - i as u32))
        .sum();

    let check_digit2 = (sum2 * 10) % 11;
    let check_digit2 = if check_digit2 == 10 { 0 } else { check_digit2 };

    check_digit2 == cpf_digits[10]
}
