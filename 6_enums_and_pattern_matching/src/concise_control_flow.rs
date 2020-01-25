pub fn main() {
  // If let is another way to accomplish what is done with match
  // It's meant for cases that are too verbose to implement with match
  let some_u8_value = Some(0u8);
  match some_u8_value {
    Some(3) => println!("three!"),
    _ => println!("not three!")
  }

  if let Some(3) = some_u8_value {
    println!("three!");
  } else {
    println!("it was {:?}", some_u8_value);
  }
}