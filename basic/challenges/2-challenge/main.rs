use std::io;

// Função que simula a transferência e retorna um Result com mensagem apropriada
fn transferir(saldo: i32, valor: i32) -> Result<(), &'static str> {
    // TODO: Implemente a verificação do valor e do saldo, retornando o Result correto
    // Dica: Use if para checar se o valor é inválido ou se o saldo é insuficiente.
    if valor <= 0 {
        Err("Valor invalido")
    } else if saldo < valor {
        Err("Saldo insuficiente")
    } else {
        Ok(())
    }
}

fn main() {
    // Lê o saldo da conta de origem
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let saldo: i32 = input.trim().parse().unwrap();

    // Lê o valor a ser transferido
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let valor: i32 = input.trim().parse().unwrap();

    // Usa o controle de fluxo com Result para decidir a mensagem
    match transferir(saldo, valor) {
        Ok(_) => println!("Transferencia realizada"),
        Err(msg) => println!("{}", msg),
    }
}
