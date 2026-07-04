use std::io;

use validator as vd;

fn main() {
    println!("Type a CPF to validate:");

    let mut cpf = String::new();

    match io::stdin().read_line(&mut cpf) {
        Ok(_) => {
            println!("You entered: {}", cpf.trim());
        },
        Err(error) => println!("Error reading input: {}", error),
    }

    let validated_cpf = vd::validate_cpf(cpf.as_str());

    if validated_cpf {
        println!("CPF is valid.");
    } else {
        println!("CPF is invalid.");
    }
}
