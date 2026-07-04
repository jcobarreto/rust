#[macro_use]
mod create_struct;

create_struct! {
  Client {
    id: u32,
    name: String,
    cpf: String,
  }

  fn show_id(&self) -> String {
    format!("ID: {}", self.id)
  }

  fn show_name(&self) -> String {
    format!("Name: {}", self.name)
  }

  fn show_cpf(&self) -> String {
    format!("CPF: {}", self.cpf)
  }

  fn add_number_to_id(&self, number: u32) -> String {
    format!("ID + {}: {}", number, self.id + number)
  }
}

fn main() {
    // Agora podemos usar a struct Cliente após sua definição
    let client = Client {
        id: 1,
        name: "João da Silva".to_string(),
        cpf: "123.456.789-00".to_string(),
    };

    println!("{}", client.show_id());
    println!("{}", client.show_name());
    println!("{}", client.show_cpf());
    println!("{}", client.add_number_to_id(10));
}
