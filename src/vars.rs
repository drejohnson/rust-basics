pub fn run() {
  let name = "Dre";
  let mut age = 41;
  age = 42;
  println!("My name is {} and I'm {}", name, age);

  // Define constant
  const ID: i32 = 007;
  println!("ID: {}", (ID));

  // Assign multiple variables
  let (my_name, my_age) = ("Dre", 21);
  println!("my_name: {}, my_age: {}", my_name, my_age);
}