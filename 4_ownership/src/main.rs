fn main() {
  let s1 = String::from("hello"); // s1 comes into scope
  let s1_len = calculate_string_len(&s1); // Pass a reference instead of moving the string
  println!("s1 is still in scope! {}", s1); // s1 is still in scope because we didn't move it
  println!("length of s1 is {}", s1_len);

  let mut s1 = String::from("hello"); // Make mutable string
  println!("s1 is {}", s1);
  borrow_and_mutate(&mut s1); // To mutate a borrowed string it needs to be typed correctly
  println!("s1 is now {}", s1);

  // Immutable values can be borrowed multiple times
  let reference_1 = &s1;
  let reference_2 = &s1;
  println!("reference_1 is {} and reference_2 is also {}.", reference_1, reference_2);

  // mutable values can be borrowed only once
  let reference_1 = &mut s1;
  println!("reference_1 is now mutable, {}", reference_1);
  let reference_2 = &mut s1;
  // uncmmenting println! below will throw an error
  // println!("reference_1 is now mutable, {}", reference_1);
  
  // now that s1 is borrowed as immutable it cannot be borrowed as mutable.
  let reference_1 = &s1; 
  let reference_2 = &s1;
  let reference_3 = &mut s1;
  // uncomment below to throw an error
  // println!("reference_1 is {}, reference_2 is {}, reference_3 is {}", reference_1, reference_2, reference_3);
  // however if reference_1 and reference_2 are not used after this point it's ok to call reference_3 here
  println!("reference_3 is mutable and it's value is {}", reference_3);
}

fn calculate_string_len(string: &String) -> usize { // string comes into scope
  let len = string.len(); // len comes into scope
  len
} // string goes out of scope but is not dropped because function doesn't have ownership. len goes out of scope and is dropped.

fn borrow_and_mutate(string: &mut String) {
  string.push_str(", world");
}

// rust will prevent you from using this function since it tries to return a reference to a variable that is already out of scope
// uncomment below to throw an error
// fn dangle() -> &String {
//   let dangling_string = String::from("dangling string"); // string in scope
//   &dangling_string // return reference
// } // string out of scope