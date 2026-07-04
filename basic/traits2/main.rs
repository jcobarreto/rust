trait Speak {
  fn speak(&self);
}

struct Dog;

impl Speak for Dog {
  fn speak(&self) {
    println!("Woof!");
  }
}

struct Cat;

impl Speak for Cat {
  fn speak(&self) {
    println!("Meow!");
  }
}

fn main() {
  let dog = Dog;
  let cat = Cat;

  dog.speak();
  cat.speak();
}
