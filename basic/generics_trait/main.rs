// fn quantidade_caracteres_inteiro(i: i32) -> usize {
//   i.to_string().chars().count()
// }

// fn quantidade_caracteres_float(f: f64) -> usize {
//   f.to_string().chars().count()
// }

// fn quantidade_caracteres_strings(s: &str) -> usize {
//   s.chars().count()
// }

trait ContaCaracteres {
  fn conta_caracteres(&self) -> usize;
}

impl ContaCaracteres for i32 {
  fn conta_caracteres(&self) -> usize {
    self.to_string().chars().count()
  }
}

impl ContaCaracteres for f64 {
  fn conta_caracteres(&self) -> usize {
    self.to_string().chars().count()
  }
}

impl<'a> ContaCaracteres for &'a str {
  fn conta_caracteres(&self) -> usize {
    self.chars().count()
  }
}

fn quantidade_caracteres<T: ContaCaracteres>(valor: T) -> usize {
  valor.conta_caracteres()
}

fn main() {
  let int_val = 12345;
  let float_val: f64 = 123.45;
  let string_val: &str = "Olá, Rust!";

  // println!("Quantidade de caracteres no inteiro: {}", quantidade_caracteres_inteiro(int_val));
  // println!("Quantidade de caracteres no float: {}", quantidade_caracteres_float(float_val));
  // println!("Quantidade de caracteres na string: {}", quantidade_caracteres_strings(string_val));

  println!("Quantidade de caracteres no inteiro: {}", quantidade_caracteres(int_val));
  println!("Quantidade de caracteres no float: {}", quantidade_caracteres(float_val));
  println!("Quantidade de caracteres na string: {}", quantidade_caracteres(string_val));
}
