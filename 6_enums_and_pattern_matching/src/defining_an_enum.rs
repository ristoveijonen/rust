enum IpAddressType {
  V4,
  V6,
}

// Unlike Structs, enums can have multiple types inside
enum IpAddress {
  V4(String),
  V4_2(u8, u8, u8, u8),
  V6(String)
}

#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    println!("{:?}", self);
  }
}

// This function can take both V4 and V6 as parameter
fn route (ip_type: IpAddressType) {
}

pub fn main() {
  let four = IpAddressType::V4;
  let six = IpAddressType::V6;
  route(four);
  route(six);

  let home = IpAddress::V4(String::from("127.0.0.1"));
  let home = IpAddress::V4_2(127, 0, 0, 1);
  let loopback = IpAddress::V6(String::from("::1"));

  let m = Message::Write(String::from("Hello!"));
  m.call();
  println!("{:?}", m);

  // Rust does not have None type in the sense that any value can have or not have value.
  // Rusts Option<T> allows having some() value or None
  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_number: Option<i32> = None;

  // In other words if a type is not Option, it must have a value.
  let number: i8 = 13;
  let some_number: Option<i8> = Option::Some(14);
  // Adding the two above throws an error
  // let value = number + some_number;
}
