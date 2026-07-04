trait Display {
  fn display(&self) -> String;
}

struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn mostrar(&self) -> String {
    format!("Point(x: {}, y: {})", self.x, self.y)
  }
}

impl Display for Point {
  fn display(&self) -> String {
    format!("Point(x: {}, y: {})", self.x, self.y)
  }
}

fn print_display(item: &impl Display) {
  println!("{}", item.display());
}

fn main() {
  let point = Point { x: 5, y: 10 };
  point.mostrar();
  point.display();

  print_display(&point);
}