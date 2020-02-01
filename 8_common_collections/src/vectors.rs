#[derive(Debug)]
enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

pub fn vectors() {
  // Vectors can only store one type of items
  // It stoes the data next to each other in the heap
  let v: Vec<i32> = Vec::new();
  println!("{:?}", v);

  // Rust infers the type for vectors automatically, the above has no initial value so it needs to be explicitly typed.
  // However if we use the vec! and add some values, rust will type it for us under the hood.
  let mut v = vec![1, 2, 32];
  println!("{:?}", v);

  // Values can be pushed into vectors
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(8);
  println!("{:?}", v);

  // Vector values can be accessed in two ways
  // Accessing the index of the vector &v[2], & must be used here and we get a reference as a return value (&).
  // If no value at index 2, the program panics and crashes.
  let third: &i32 = &v[2];
  println!("The third element is {}", third);

  // The other way is v.get(2), we once again get the value by index, but get back Option<T> as a return value.
  // Here we can handle the possibility of no value at index 2.
  match v.get(2) {
      Some(third) => println!("The third element is {}", third),
      None => println!("There is no third element."),
  }

  // Rust does not allow mutable & immutable borrows from the same vector.
  // Even though the immutable borrow is to the start of the vector and change happesn to the end, rust does not allow this, because it might have to allocate the whole vector to a new spot in memory if the new value does not have space next to the existing vector.
  let mut v = vec![1, 2, 3, 4, 5];
  // let first = &v[0];
  v.push(6);
  // println!("The first element is: {}", first);

  // Looping a vector
  // It is possible to loop mutable values fo a vector.
  for i in &mut v {
    // Dereference operator (*) needs to be used to change the value i is referencing to.
    *i += 50;
    println!("{}", i);
  }

  let v = vec![1, 2, 3, 4, 5, 6];
  for i in &v {
    // it is possible to loop a immutable values of a vector.
    println!("{}", i);
  }

  // enums can be used to store multiple values inside a vector
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
  for cell in &row {
    println!("{:?}", &row);
  }
} // --> v is out of scope