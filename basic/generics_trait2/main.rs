use std::fmt::Display;

fn quantidade_caracteres<T: Display>(valor: T) -> usize {
  valor.to_string().chars().count()
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
