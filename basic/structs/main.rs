#[derive(Debug)]
struct Cliente {
    id: u32,
    nome: String,
    cpf: String,
}

impl Cliente {
  fn cpf_valido(&self) -> bool {
    if self.cpf.is_empty() {
      return false;
    }
    return true;
  }
}

fn main() {
    let cliente = Cliente {
        id: 1,
        nome: String::from("João Silva"),
        cpf: String::from("123.456.789-00"),
    };

    let valido: &str = if cliente.cpf_valido() {
      "válido"
    } else {
      "inválido"
    };

    println!(
      "O CPF do cliente ({}): {} é {} \n{:#?}",
      cliente.id,
      cliente.nome,
      valido,
      cliente
    );
}
