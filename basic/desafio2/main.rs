use std::io;

fn main() {
    // Lê a linha de entrada do usuário
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");

    // Divide a entrada em partes e faz o parse dos valores
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    // TODO: Verifique se a entrada possui exatamente 3 partes (saldo, operação, valor)

    // Dica: Use match para tratar as operações "deposit" e "withdraw"
    // Se for "deposit", some o valor ao saldo e imprima o resultado
    // Se for "withdraw", verifique se há saldo suficiente antes de subtrair e imprimir
    // Caso contrário, imprima "Insufficient funds"

    if parts.len() != 3 {
      println!("Invalid value for saldo_inicial");
      return;
    }

    let mut saldo = parts[0].parse::<f64>().expect("Erro ao parsear saldo");
    let tipo_operacao = parts[1];
    let valor_operacao = parts[2].parse::<f64>().expect("Erro ao parsear valor da operação");

    if saldo < 0.0 {
      println!("Invalid value for saldo_inicial");
      return;
    } else if valor_operacao <= 0.0 {
      println!("Invalid value for valor_operacao");
      return;
    }

    match tipo_operacao {
      "deposit" => saldo += valor_operacao,
      "withdraw" => {
        if valor_operacao > saldo {
          println!("Insufficient funds");
          return;
        }
        saldo -= valor_operacao
      },
      _ => {
        println!("Invalid operation type");
        return;
      }
    }
  println!("{}", saldo);
}
