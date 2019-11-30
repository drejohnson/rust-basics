pub fn run() {
  // Print to console
  println!("Hello from the print.rs file");

  // Basic formatting
  println!("{} is from {}", "Dre", "Chicago");

  // Positional Arguments
  println!("{0} is from {1} and {0} like to {2}", "Dre", "Chicago", "code");

  // named Arguments
  println!("{name} likes to {activity}", name = "Dre", activity = "code");

  // Placeholder Traits
  println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (12, true, "hello"));
}