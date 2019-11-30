pub fn run() {
  let hello = "Hello";

  let mut hello_world = String::from(hello);

  // Push string
  hello_world.push_str(" World");

  // Capacity in bytes
  println!("Capacity: {}", hello_world.capacity());

  // Contains
  println!("Contains word World: {}", hello_world.contains("World"));

  // Get length
  println!("Length {}", hello.len());

  // Loop through sting by whitespace
  for word in hello_world.split_whitespace() {
    println!("{}", word);
  }

  // Create string with capacity
  let mut s = String::with_capacity(10);
  s.push_str("I know Rust");

  // Assertion testing
  assert_eq!(9, s.len());
  assert_eq!(11, s.capacity());

  println!("{}", s);

  println!("{}", hello_world);

  println!("{}", hello);
}
