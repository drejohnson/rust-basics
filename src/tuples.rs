pub fn run() {
  let person: (&str, i32, &str) = ("Dre", 41, "Shanghai");

  println!(
    "{} is {} years old and lives in {}",
    person.0, person.1, person.2
  )
}
