// Resizble Arrays

use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // Re-assign value
  numbers[2] = 20;

  // Add on to vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value
  numbers.pop();

  println!("{:?}", numbers);

  // Get single value
  println!("Single value: {}", numbers[0]);

  // Get vector length
  println!("Vector length: {}", numbers.len());

  // Vectors are stack allocated
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for n in numbers.iter() {
    println!("{}", n);
  }

  // Loop and mutate values
  for n in numbers.iter_mut() {
    *n *= 2;
    println!("{}", n);
  }

  println!("{:?}", numbers);
}
