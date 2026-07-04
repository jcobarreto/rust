trait Pessoa {
  fn mostra(&self);
}

struct PessoaFisica {
  id: u32,
  nome: String,
  cpf: String,
}

impl Pessoa for PessoaFisica {
  fn mostra(&self) {
    println!("\
      ID: {}\n\
      Nome: {}\n\
      CPF: {}\n\
      ", self.id,
      self.nome,
      self.cpf,
    )
  }
}

struct PessoaJuridica {
  id: u32,
  nome: String,
  cnpj: String,
}

impl Pessoa for PessoaJuridica {
  fn mostra(&self) {
    println!("\
      ID: {}\n\
      Nome: {}\n\
      CNPJ: {}\n\
      ", self.id,
      self.nome,
      self.cnpj,
    )
  }
}

fn exibe_documento(pessoa: &dyn Pessoa) {
  pessoa.mostra();
}

fn main() {
  let pf = PessoaFisica {id: 1, nome: String::from("João"), cpf: String::from("123.456.789-00"),};
  let pj = PessoaJuridica {id: 2, nome: String::from("Empresa XYZ"), cnpj: String::from("12.345.678/0001-00"),};

  exibe_documento(&pf);
  exibe_documento(&pj);
}
