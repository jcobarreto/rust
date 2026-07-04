#[derive(PartialEq)]
enum Tipo {
  Fisica,
  Juridica,
}

struct Pessoa {
  nome: String,
  documento: String,
  tipo: Tipo,
}

fn main() {
  let daniel = Pessoa {
    nome: String::from("João Silva"),
    documento: String::from("99.999.999/0001-99"),
    tipo: Tipo::Juridica,
  };

  // // Pattern Matching
  // match daniel.tipo {
  //   Tipo::Fisica => {
  //     println!("{} é uma pessoa física", daniel.nome);
  //   },
  //   _ => {
  //     println!("{} é uma pessoa jurídica", daniel.nome);
  //   }
  // }

  if daniel.tipo == Tipo::Fisica {
    println!("{} é uma pessoa física", daniel.nome);
  } else {
    println!("{} é uma pessoa jurídica", daniel.nome);
  }
}
