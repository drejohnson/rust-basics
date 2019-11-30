use std::env;

pub fn run() {
  let args: Vec<String> = env::args().collect();
  let cmd = args[1].clone();
  let name = "Dre";
  let status = "100%";

  // println!("Command {}", cmd);

  if cmd == "hello" {
    println!("Hi {}, how are you?", name)
  } else if cmd == "status" {
    println!("Status is {}", status);
  } else {
    println!("That is not a valid command");
  }
}
