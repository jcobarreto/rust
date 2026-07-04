use std::io;

fn main() {
    // Lê a linha de entrada do usuário
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        println!("Erro: entrada invalida");
        return;
    }

    // Divide a entrada em partes separadas por espaço
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        println!("Erro: entrada invalida");
        return;
    }

    // Tenta converter o saldo para inteiro positivo
    let saldo: i32 = match parts[0].parse() {
        Ok(n) if n >= 0 => n,
        _ => {
            println!("Erro: entrada invalida");
            return;
        }
    };

    // Tenta converter o valor do saque para inteiro
    let saque: i32 = match parts[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Erro: entrada invalida");
            return;
        }
    };

    // TODO: Verifique se o valor do saque é válido (> 0) e se há saldo suficiente. Imprima o novo saldo ou a mensagem de erro apropriada.
    if saque <= 0 {
        println!("Erro: valor invalido");
    } else if saque > saldo {
        println!("Erro: saldo insuficiente");
    } else {
        println!("{}", saldo - saque);
    }
}
