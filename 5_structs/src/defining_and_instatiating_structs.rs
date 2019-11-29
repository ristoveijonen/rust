// Structs are something similar to objects in other languages
#[derive(Debug)]
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

// structs can be used to create "tuple structs", which means a struct without any name fields
// the structs below are like tuples with their own names and types
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

pub fn main() {
  let username = String::from("someusername123");
  let email = String::from("someone@example.com");
  // declaring mutable User struct
  // note that structs are always mutable or immutable, no mixed mutability allowed inside one struct
  let mut user1 = build_user(username, email);
  println!("user1: {:#?}", user1);

  // values from structs can be accessed with dots
  user1.email = String::from("another@example.com"); // value can be changes because user1 is mutable
  println!("email of user1 was changed to {}", user1.email);

  // the ..user1 tells rust to populate user2 with the fileds it's missing from user1 and to give those fields same values as user1 has
  let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
  };
  println!("user2 is {:#?}", user2);

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
  println!(
    "black {:?} and origin {:?} are quite similar...",
    black, origin
  );
}

fn build_user(username: String, email: String) -> User {
  User {
    username, // shorthand for decalring struct value -> username: username
    email,    // email: email
    sign_in_count: 1,
    active: true,
  }
}
