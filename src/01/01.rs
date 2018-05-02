fn main() {
  use std::io;

  println!("What is your name?");

  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();

  let message = "Hello,".to_string()+input.trim()+",nice to meet you!";

  println!("{}",message);
}