// Used to create custom data types

// Traditional struct
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// Tuple struct
struct Rgb(u8, u8, u8);

struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Construct Person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }
  // Get fullname
  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string();
  }

  fn to_tuple(self) -> (String, String) {
    (self.first_name, self.last_name)
  }
}

pub fn run() {
  let mut c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };

  let mut r = Rgb(255, 0, 0);

  r.0 = 200;

  let mut p = Person::new("Dre", "Johnson");
  // println!("New Person: {} {}", p.first_name, p.last_name);
  println!("Persons Fullname: {}", p.full_name());
  p.set_last_name("Ingram");
  println!("Persons Fullname: {}", p.full_name());

  println!("Person as tuple: {:?}", p.to_tuple());

  // println!("RGB: {}, {}, {}", c.red, c.green, c.blue);

  // println!("RGB: {}, {}, {}", r.0, r.1, r.2);
}
