fn contar_posicoes<T>(array: &[T]) -> usize {
  array.len()
}

fn main() {
  let array_inteiros = [1, 2, 3, 4, 5];
  let array_floats = [1.1, 2.2, 3.3, 4.4, 5.5];
  let array_strings = ["Olá", "Mundo", "Rust"];

  println!("Posições no array de inteiros: {}", contar_posicoes(&array_inteiros));
  println!("Posições no array de floats: {}", contar_posicoes(&array_floats));
  println!("Posições no array de strings: {}", contar_posicoes(&array_strings));
}
