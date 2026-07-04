use std::io;

fn main() {
    // Lê a linha de entrada do usuário
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler entrada");

    // Divide a entrada em partes e remove espaços extras
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 3 {
      println!("Invalid values");
      return;
    }

    let conta_origem = parts[0];
    let conta_destino = parts[1];
    let valor_operacao = parts[2].parse::<f64>().expect("Erro ao parsear valor da operação");

    if conta_origem.len() != 6 || conta_destino.len() != 6 || conta_origem == conta_destino {
      println!("REJEITADA");
      return;
    } else if valor_operacao <= 0.0 {
      println!("REJEITADA");
      return;
    }

    // TODO: Verifique se as regras de validação da transferência são atendidas
    // - As contas devem ter 6 dígitos, ser diferentes e o valor deve ser inteiro positivo (>0)
    // Dica: Use métodos como len(), chars().all(), parse::<i32>() e comparações para validar.

    // Exemplo de saída (substitua pela lógica de validação):
    println!("APROVADA");
}
