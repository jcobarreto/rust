use std::collections::HashMap;

fn main() {
  let mut dados: HashMap<&str, i32> = HashMap::new();

  dados.insert("Largura", 10);
  dados.insert("Altura", 50);

  dados.entry("Media").or_insert(30);
  let media = dados.entry("Media").or_insert(30);
  println!("{}", media);

  println!("{:#?}", dados);
}
