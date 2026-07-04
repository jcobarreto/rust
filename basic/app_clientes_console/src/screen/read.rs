use std::io;

pub fn read_data() -> String {
  let mut data = String::new();
  io::stdin().read_line(&mut data).expect("Failed to read line");
  data.trim().to_string()
}

pub fn read_data_int() -> usize {
  let mut data = String::new();
  io::stdin().read_line(&mut data).expect("Failed to read line");
  data.trim().parse().expect("Error converting to integer")
}
